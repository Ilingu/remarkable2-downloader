use anyhow::{anyhow, Result};
use std::{fs, time::Duration};

use colored::Colorize;

pub fn print_err(err: &str) {
    eprintln!("{}", err.red().bold());
}

#[macro_export]
macro_rules! udp_return {
    ($result:expr, $udp_mode:expr, $error_msg:expr) => {
        match $result {
            Ok(d) => d,
            Err(_) => {
                if $udp_mode {
                    println!(
                        "{}",
                        "[UDP_MODE]: a subfolder as been entirely skipped due to an error".yellow()
                    );
                    return Ok(());
                } else {
                    return Err(anyhow!($error_msg));
                }
            }
        }
    };
}

#[macro_export]
macro_rules! udp_continue {
    ($result:expr, $udp_mode:expr, $error_msg:expr) => {
        match $result {
            Ok(d) => d,
            Err(why) => {
                if $udp_mode {
                    println!(
                        "{}",
                        "[UDP_MODE]: a file as been skipped due to an error".yellow()
                    );
                    continue;
                } else {
                    return Err(anyhow!(format!("{}: {why}", $error_msg)));
                }
            }
        }
    };
}

pub trait UnwrapExtra<T> {
    fn unwrap_colored(self, err: &str) -> T;
}

impl<T, E: std::fmt::Debug> UnwrapExtra<T> for Result<T, E> {
    fn unwrap_colored(self, err: &str) -> T {
        print_err(err);
        self.unwrap()
    }
}

pub async fn is_client_up() -> bool {
    let client = reqwest::Client::new();
    let resp = client
        .get("http://10.11.99.1")
        .timeout(Duration::from_secs(5))
        .send()
        .await;
    match resp {
        Ok(resp) => resp.status().as_u16() == 200,
        Err(_) => false,
    }
}

/// check if the output path exist and if it exist check if it's a directory
pub fn is_dir(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

pub fn check_output_path(path: &str, allow_creation: bool) -> Result<()> {
    if !is_dir(path) {
        match allow_creation {
            true => {
                if fs::create_dir_all(path).is_err() {
                    return Err(anyhow!(""));
                }
            }
            false => return Err(anyhow!("")),
        }
    }
    Ok(())
}

pub fn ensure_file_extension(name: &str) -> String {
    if name.ends_with(".pdf") {
        name.to_string()
    } else {
        format!("{name}.pdf")
    }
}
