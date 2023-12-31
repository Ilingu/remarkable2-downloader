use std::{fs, path::Path, str::FromStr};

use anyhow::{anyhow, Result};
use chrono::DateTime;
use colored::Colorize;

use crate::{
    cmd::FolderNode,
    scheme::{DocType, RmkDocument, RmkFile},
    udp_continue,
    utils::ensure_file_extension,
};

use super::{copy_to_localfs, RemarkableFSHierarchy};

pub struct BackupOptions {
    pub out_path: String,
    pub udp_mode: bool,
    pub override_mode: bool,
    pub smart_mode: bool,
}

fn which_files_to_download(
    fs_hierarchy: &RemarkableFSHierarchy,
    out_path: &str,
    smart_mode: bool,
) -> Vec<(String, String)> {
    if !smart_mode {
        let files_to_download = fs_hierarchy
            .all_docs
            .iter()
            .filter_map(
                |RmkDocument {
                     id,
                     vissible_name,
                     doc_type,
                     ..
                 }| match doc_type {
                    DocType::DocumentType => Some((id.to_owned(), vissible_name.to_owned())),
                    _ => None,
                },
            )
            .collect::<Vec<_>>();
        return files_to_download;
    }

    fn compare_files_time(
        folder_hierarchy: &FolderNode,
        files: &[RmkDocument],
        path: &str,
        add_all: bool,
    ) -> Vec<(String, String)> {
        // update path to the subfolder
        let curr_path = &format!(
            "{path}{}{}",
            if path.ends_with('/') { "" } else { "/" },
            folder_hierarchy.name
        );
        if add_all || !Path::new(curr_path).exists() {
            let mut file_to_download = files
                .iter()
                .filter_map(
                    |RmkDocument {
                         id, vissible_name, ..
                     }| match folder_hierarchy.files_id.contains(id) {
                        true => Some((id.to_owned(), vissible_name.to_owned())),
                        false => None,
                    },
                )
                .collect::<Vec<_>>();
            for subdir in &folder_hierarchy.subfolders {
                let mut sub_files = compare_files_time(subdir, files, curr_path, true);
                file_to_download.append(&mut sub_files);
            }

            return file_to_download;
        }

        let mut files_to_download = files
            .iter()
            .filter_map(
                |RmkDocument {
                     id,
                     modified_client,
                     vissible_name,
                     ..
                 }| {
                    let file_to_dl = (id.to_owned(), vissible_name.to_owned());
                    match folder_hierarchy.files_id.contains(id) {
                        true => {
                            let rmkfile_elapsed = match DateTime::<chrono::Local>::from_str(modified_client) {
                                Ok(d) => match (chrono::Local::now() - d).to_std() {
                                    Ok(elasped) => elasped,
                                    Err(_) => return Some(file_to_dl),
                                }
                                Err(_) => return Some(file_to_dl),
                            };

                            let path = format!("{curr_path}/{vissible_name}");
                            let localfile_elapsed = match fs::metadata(path) {
                                Ok(metadata) => match metadata.modified() {
                                    Ok(creation_date) => match creation_date.elapsed() {
                                        Ok(elasped) => elasped,
                                        Err(_) => return Some(file_to_dl),
                                    },
                                    Err(_) => return Some(file_to_dl),
                                },
                                Err(_) => return Some(file_to_dl),
                            };

                            if rmkfile_elapsed <= localfile_elapsed {
                                println!("{}", format!("[SMART_MODE]: adding '{vissible_name}', because change made since last download").purple());
                                Some(file_to_dl)
                            } else {
                                println!("{}", format!("[SMART_MODE]: skipped '{vissible_name}', because no change made since last download").yellow());
                                None
                            }
                        }
                        false => None,
                    }
                },
            )
            .collect::<Vec<_>>();

        // create subfolders
        for subfolder_hierarchy in &folder_hierarchy.subfolders {
            let mut subfiles = compare_files_time(subfolder_hierarchy, files, curr_path, false);
            files_to_download.append(&mut subfiles);
        }

        files_to_download
    }

    let all_files = fs_hierarchy
        .all_docs
        .iter()
        .filter_map(|doc| match doc.doc_type {
            DocType::DocumentType => Some(RmkDocument {
                vissible_name: ensure_file_extension(&doc.vissible_name),
                ..doc.clone()
            }),
            _ => None,
        })
        .collect::<Vec<_>>();

    compare_files_time(&fs_hierarchy.folder_hierarchy, &all_files, out_path, false)
}

pub async fn sync_full_backup(
    fs_hierarchy: &RemarkableFSHierarchy,
    BackupOptions {
        out_path,
        udp_mode,
        override_mode,
        smart_mode,
    }: BackupOptions,
) -> Result<()> {
    let files_to_download = which_files_to_download(fs_hierarchy, &out_path, smart_mode);

    let total_download = files_to_download.len();
    if total_download == 0 && smart_mode {
        println!(
            "{}",
            "[SMART MODE]: No change made since last backup, exiting... (PS: set smart_mode to false if you still want to download)".green()
        );
        return Ok(());
    }

    println!(
        "{}",
        format!("Downloading {total_download} files... (This may take a (very) long time)").blue()
    );

    let client = reqwest::Client::new();
    let mut files: Vec<RmkFile> = vec![];
    for (id, name) in files_to_download {
        let name = ensure_file_extension(&name);
        println!("{}", format!("Downloading {name}...").purple());
        let resp = client
            .get(format!("http://10.11.99.1/download/{id}/placeholder"))
            .send()
            .await?;
        let b = udp_continue!(resp.bytes().await, udp_mode, "").to_vec();
        files.push((id, name, b))
    }

    println!(
        "{}",
        "Copying downloaded files to local file system...".blue()
    );
    copy_to_localfs(
        &fs_hierarchy.folder_hierarchy,
        &files,
        &out_path,
        udp_mode,
        override_mode,
    )?;
    println!(
        "{}",
        format!("Finished copying file, go see: '{out_path}'").green()
    );

    Ok(())
}

/* [DEAD CODE]: remarkable does not support concurrent requests
pub async fn async_full_backup(
    fs_hierarchy: &RemarkableFSHierarchy,
    BackupOptions {
        out_path,
        udp_mode,
        override_mode,
        smart_mode,
    }: BackupOptions,
    concurrent_requests: usize,
) -> Result<()> {
    let files_to_download = which_files_to_download(fs_hierarchy, &out_path, smart_mode);
    let total_download = files_to_download.len();

    println!(
        "{}",
        format!("Downloading {total_download} files...").blue()
    );
    let client = reqwest::Client::new();
    let file_bdatas = stream::iter(files_to_download)
        .map(|(id, name)| {
            let client = &client;
            async move {
                let resp = client
                    .get(format!("http://10.11.99.1/download/{id}/placeholder"))
                    .send()
                    .await?;
                resp.bytes().await.map(|b| (id, name, b.to_vec()))
            }
        })
        .buffer_unordered(concurrent_requests);

    let files = file_bdatas
        .filter_map(|file| async { file.ok() })
        .collect::<Vec<RmkFile>>()
        .await;

    let success = files.len();
    let failure = total_download - success;
    if failure != 0 && !udp_mode {
        return Err(anyhow!("Failure, at least on download failed".red().bold()));
    }

    println!(
        "Successful download: {}",
        format!("{success}/{total_download}").green()
    );
    println!(
        "Failed download: {}",
        format!("{failure}/{total_download}").red()
    );

    println!(
        "{}",
        "Copying downloaded files to local file system...".blue()
    );
    copy_to_localfs(
        &fs_hierarchy.folder_hierarchy,
        &files,
        &out_path,
        udp_mode,
        override_mode,
    )?;
    println!(
        "{}",
        format!("Finished copying file, go see: '{out_path}'").green()
    );

    Ok(())
}*/
