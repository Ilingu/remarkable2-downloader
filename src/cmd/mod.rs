use async_recursion::async_recursion;
use colored::Colorize;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    time::Duration,
};

use crate::{
    scheme::{DocType, RmkDocument, RmkDocuments, RmkFile},
    udp_continue, udp_return,
};
use anyhow::{anyhow, Result};

pub mod download;
pub mod full_backup;

#[derive(Debug)]
pub struct FolderNode {
    pub name: String,
    pub id: String,
    pub files_id: Vec<String>,
    pub subfolders: Vec<FolderNode>,
}

pub struct RemarkableFSHierarchy {
    pub all_docs: RmkDocuments,
    pub folder_hierarchy: FolderNode,
}

#[async_recursion(?Send)]
pub async fn fetch_documents(id: &str, name: &str) -> Result<RemarkableFSHierarchy> {
    let mut all_docs: RmkDocuments = vec![];
    let mut subfolders_hierarchy: Vec<FolderNode> = vec![];

    let client = reqwest::Client::new();
    let mut docs = client
        .get(format!("http://10.11.99.1/documents/{}", id))
        .timeout(Duration::from_secs(1))
        .send()
        .await?
        .json::<RmkDocuments>()
        .await?;

    let sub_folder = docs
        .iter()
        .filter(|RmkDocument { doc_type, .. }| doc_type == &DocType::CollectionType);
    for RmkDocument {
        id, vissible_name, ..
    } in sub_folder
    {
        let mut sub_hierarchy = fetch_documents(id, vissible_name).await?;
        subfolders_hierarchy.push(sub_hierarchy.folder_hierarchy);
        all_docs.append(&mut sub_hierarchy.all_docs);
    }

    let folder_hierarchy = FolderNode {
        name: name.to_string(),
        id: id.to_string(),
        files_id: docs
            .iter()
            .filter_map(|RmkDocument { id, doc_type, .. }| match doc_type {
                DocType::DocumentType => Some(id.to_owned()),
                _ => None,
            })
            .collect::<_>(),
        subfolders: subfolders_hierarchy,
    };
    all_docs.append(&mut docs);

    Ok(RemarkableFSHierarchy {
        all_docs,
        folder_hierarchy,
    })
}

pub fn copy_to_localfs(
    folder_hierarchy: &FolderNode,
    files: &[RmkFile],
    path: &str,
    udp_mode: bool,
    override_mode: bool,
) -> Result<()> {
    // update path to the subfolder
    let curr_path = &format!(
        "{path}{}{}",
        if path.ends_with('/') { "" } else { "/" },
        folder_hierarchy.name
    );
    // create folder if does not exist
    if !Path::new(curr_path).exists() {
        let task = fs::create_dir(curr_path);
        udp_return!(task, udp_mode, "Failed to create dir".red());
    }

    // copy current folder level files
    let files_to_copy = files
        .iter()
        .filter(|(id, _, _)| folder_hierarchy.files_id.contains(id));
    for (_, name, bytes) in files_to_copy {
        let path = format!("{curr_path}/{name}");
        if override_mode {
            let task = fs::write(path, bytes);
            udp_continue!(task, udp_mode, "Failed to write file".red());
        } else {
            let mut file = OpenOptions::new()
                .write(true)
                .append(false)
                .truncate(true)
                .create_new(true)
                .open(path)?;
            let task = file.write_all(bytes);
            udp_continue!(task, udp_mode, "Failed to write file".red());
        }
    }

    // create subfolders
    for subfolder_hierarchy in &folder_hierarchy.subfolders {
        let task = copy_to_localfs(
            subfolder_hierarchy,
            files,
            curr_path,
            udp_mode,
            override_mode,
        );
        udp_continue!(task, udp_mode, "Failed to copy subfolder".red());
    }

    Ok(())
}
