#[macro_use]
extern crate wei_log;

mod action;

use std::env;

use bollard::errors::Error;
use bollard::image::{CreateImageOptions,ListImagesOptions};
use bollard::container::{ListContainersOptions};
use bollard::Docker;

use std::io;
use std::default::Default;
use std::collections::HashMap;
use std::process::Command;

use futures::StreamExt;

use wei_download::DownloadMethod::QBitTorrent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    wei_env::bin_init("wei-docker");
    let args: Vec<String> = env::args().collect();

    let mut command = "".to_string();
    if args.len() > 1 {
        command = args[1].clone();
    }

    match command.as_str() {
        "download" => {
            info!("Downloading...");
            wei_download::add(
                QBitTorrent, 
                "http://download.zuiyue.com/windows/torrent/docker.torrent",
                &std::env::current_dir()?.display().to_string()
            )?;
        }
        "install" => {
            info!("Installing...");
            wei_docker_install::install();
        },
        "start" => {
            info!("Starting...");
            action::start();
        },
        "stop" => {
            info!("Stoping...");
            action::stop();
        }
        "uninstall" => {
            println!("Uninstalling...");
        },
        "check" => {
        },
        "api" => {
        },
        _ => {
            return Ok(());
        }
    }

    Ok(())
}

pub async fn is_runing() -> Result<(), Error> {
    match Docker::connect_with_local_defaults() {
        Ok(docker) => {
            // Check Docker daemon is running by fetching system information
            let _ = docker.version().await?;
            println!("{:?}",serde_json::json!({
                "code": 200
            }));
            
        }
        Err(_) => {
            // For Linux, macOS, and Windows
            let start_docker = if cfg!(target_os = "linux") {
                Command::new("systemctl")
                    .arg("start")
                    .arg("docker")
                    .output()
            } else if cfg!(target_os = "macos") {
                Command::new("open")
                    .arg("--background")
                    .arg("-a")
                    .arg("Docker")
                    .output()
            } else if cfg!(target_os = "windows") {
                Command::new("net")
                    .arg("start")
                    .arg("com.docker.service")
                    .output()
            } else {
                panic!("Unsupported operating system.");
            };

            match start_docker {
                Ok(_) => print!("{:?}",serde_json::json!({
                    "code": 200
                })),
                Err(err) => print!("{:?}", serde_json::json!({
                    "code": 400,
                    "status" : format!("Failed to start Docker:{}", err)
                })),
            }
        }
    }

    Ok(())
}



// pub async fn container_rm(container_name: &str) -> io::Result<()> {
//     let output = Command::new("docker").args(&["rm",container_name]).output()?;

//     if output.status.success() {
//         Ok(())
//     } else {
//         Err(io::Error::new(io::ErrorKind::Other, format!("Failed to rm container: {}", String::from_utf8_lossy(&output.stderr))))
//     }
// }

