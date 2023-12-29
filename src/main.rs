mod cmd;
mod scheme;
mod utils;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::{
    cmd::{
        fetch_documents,
        full_backup::{async_full_backup, sync_full_backup},
    },
    utils::{check_output_path, is_client_up, print_err},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "Remarkable2 Downloader")]
#[command(author = "Ilingu")]
#[command(version = "0.1")]
#[command(about = "Partial to full backup of your remarkable2 documents", long_about = None)]
#[command(propagate_version = true)]
struct RmkdwldCli {
    #[arg(long, default_value_t = false)]
    /// If set to true, when one upload/download fail the CLI will continue to upload/download the remaining files
    ///
    /// Obviously a report will be shown in case of failure of some upload/download
    udp_mode: bool,

    /// When copying the downloaded files to your local file system if a file already at the location where a downloaded file should be copied, the CLI will:
    ///
    /// - If set to false (default): halt the execution and return an error without touching at the already present file
    ///
    /// - If set to true: override everything inside it
    #[arg(long, default_value_t = false)]
    override_mode: bool,

    /// If set to true, download request will be made asynchronously to your remarkable.
    ///
    /// While it's faster (because download are made in parralel and not one by one), I do not recommand to enable it because
    ///
    /// the remarkable hardware does not support very well all these concurrent request as a consequence I cannot assure you
    ///
    /// file integrity (they may mix up, being under the wrong filename or in the wrong place or just not being there, and a lot of other not so funny things)
    ///
    /// But fear not this does not corrupt your remarkable in anyway, because mine is still working 👍
    #[arg(long, default_value_t = false)]
    async_mode: bool,

    /// If async mode set to true, it specifies the number of concurrent requests sent to your remarkable2, as said in the description of the async mode:
    ///
    /// the larger this number is the faster but higher is the chance of loosing your file integrity in the process
    ///
    /// After some testing remarkable hardware seem to support up to 3 concurrent requests, which is the default, but please try and experiment with 100 concurrents requests 😉
    ///
    /// Also setting this to 1 is equivalent to disabling async_mode since you do one request by one
    #[arg(long, default_value_t = 2)]
    concurrent_request: usize,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Upload folders/files to remarkable2
    Upload {
        /// Where is the location/path of the local file/folder you want to upload
        #[arg(long)]
        datapath: Vec<String>,
        /// path in the remarkable2, e.g: "/my_books/fantasy", "/" is for root
        #[arg(long)]
        uploadpath: String,
    },
    /// Download files from remarkable2
    Download {
        /// Paths of the files in the remarkable to download (one of the 2 options must be filled)
        #[arg(short, long)]
        paths: Option<Vec<String>>,
        /// IDs of the files to download (one of the 2 options must be filled)
        #[arg(long)]
        ids: Option<Vec<String>>,
        /// Folder location to save the downloaded files
        #[arg(short, long)]
        output_path: String,
        /// if the output path does not exist yet, allow this cli to create it for you
        #[arg(short, long, default_value_t = true)]
        allow_creation: bool,
    },
    /// Download all the files and folder from remarkable2 (Full backbup)
    Backup {
        /// Folder location to save the downloaded files
        #[arg(short, long)]
        output_path: String,
        /// if the output path does not exist yet, allow this cli to create it for you
        #[arg(short, long, default_value_t = true)]
        allow_creation: bool,
    },
    /// Search files by name
    Search {
        /// Name of the file to search
        #[arg(short, long)]
        name: String,
    },
    /// Get information on a specific file
    Info {
        /// Path of the file in the remarkable (one of the 2 options must be filled)
        #[arg(short, long)]
        path: Option<String>,
        /// ID of the file (one of the 2 options must be filled)
        #[arg(long)]
        id: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli_args = RmkdwldCli::parse();

    println!("{}", "Connecting to remarkable via USB...".bright_blue());
    if !is_client_up().await {
        print_err("[FATAL]: Web Usb port is not enable, or your remarkable2 is not plugged in");
        return Err(anyhow!("CLI exited with errors."));
    }
    println!("{}", "Connected to remarkable".green());
    println!(
        "{}",
        "Do not unplug your remarkable during transfers!".bright_blue()
    );

    // fetch the all documents for latter use (may be overkill, but simpler)
    let fs_hierarchy = match fetch_documents("", "root").await {
        Ok(hierarchy) => hierarchy,
        Err(why) => {
            // eprintln!("{why}");
            print_err("[FATAL]: Failed to fetch documents structure from your remarkable");
            return Err(anyhow!("CLI exited with errors."));
        }
    };

    match cli_args.command {
        Commands::Upload {
            uploadpath,
            datapath,
        } => {
            println!("{}", "Not implemented yet!".yellow());
        }
        Commands::Download {
            paths,
            ids,
            output_path,
            allow_creation,
        } => {
            // check_output_path(&output_path, allow_creation)?;
            println!("{}", "Not implemented yet!".yellow());
        }
        Commands::Backup {
            output_path,
            allow_creation,
        } => {
            check_output_path(&output_path, allow_creation)?;
            if cli_args.async_mode {
                println!(
                    "{}",
                    format!(
                        "Downloading with async mode and {} concurrent request. May god be with you.",
                        cli_args.concurrent_request
                    )
                    .yellow()
                );
                async_full_backup(
                    &fs_hierarchy,
                    &output_path,
                    cli_args.udp_mode,
                    cli_args.override_mode,
                    cli_args.concurrent_request,
                )
                .await?
            } else {
                sync_full_backup(
                    &fs_hierarchy,
                    &output_path,
                    cli_args.udp_mode,
                    cli_args.override_mode,
                )
                .await?
            }
        }
        Commands::Search { name } => {
            println!("{}", "Not implemented yet!".yellow());
        }
        Commands::Info { path, id } => {
            println!("{}", "Not implemented yet!".yellow());
        }
    };

    Ok(())
}
