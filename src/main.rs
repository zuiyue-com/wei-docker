#[macro_use]
extern crate wei_log;

mod action;

use std::env;

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
            println!("{}",wei_download::add(
                QBitTorrent, 
                "http://download.zuiyue.com/windows/torrent/docker.torrent",
                &std::env::current_dir()?.display().to_string()
            )?);
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
            info!("Uninstalling...");
        },
        "check" => {
            print!("{}",serde_json::json!({
                "code": "200",
                "msg": "success",
                "progress": wei_docker_install::check()
            }));
        },
        "api" => {
        },
        _ => {
            return Ok(());
        }
    }

    Ok(())
}
