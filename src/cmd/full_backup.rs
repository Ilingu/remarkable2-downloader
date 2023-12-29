use anyhow::{anyhow, Result};
use colored::Colorize;
use futures::{stream, StreamExt};

use crate::{
    scheme::{DocType, RmkDocument, RmkFile},
    udp_continue,
};

use super::{copy_to_localfs, RemarkableFSHierarchy};

pub async fn async_full_backup(
    fs_hierarchy: &RemarkableFSHierarchy,
    out_path: &str,
    udp_mode: bool,
    override_mode: bool,
    concurrent_requests: usize,
) -> Result<()> {
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
        out_path,
        udp_mode,
        override_mode,
    )?;
    println!(
        "{}",
        format!("Finished copying file, go see: '{out_path}'").green()
    );

    Ok(())
}

pub async fn sync_full_backup(
    fs_hierarchy: &RemarkableFSHierarchy,
    out_path: &str,
    udp_mode: bool,
    override_mode: bool,
) -> Result<()> {
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
    let total_download = files_to_download.len();

    println!(
        "{}",
        format!("Downloading {total_download} files... (This may take a (very) long time)").blue()
    );

    let client = reqwest::Client::new();
    let mut files: Vec<RmkFile> = vec![];
    for (id, name) in files_to_download {
        let name = if name.ends_with(".pdf") {
            name
        } else {
            format!("{name}.pdf")
        };
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
        out_path,
        udp_mode,
        override_mode,
    )?;
    println!(
        "{}",
        format!("Finished copying file, go see: '{out_path}'").green()
    );

    Ok(())
}
