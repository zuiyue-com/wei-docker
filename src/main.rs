#[macro_use]
extern crate wei_log;

mod action;
mod image;
mod container;

use std::env;

use wei_download::DownloadMethod::QBitTorrent;

fn main() -> Result<(), Box<dyn std::error::Error>> {

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
        "image_pull" => {
            result_string(image::pull(&args[2]));
        },
        "image_rmi" => {
            result_string(image::rmi(&args[2]));
        },
        "image_list" => {
            result_vec(image::list());
        },
        "image_exists" => {
            result(image::exists(&args[2]));
        },
        "container_run" => {
            result_string(container::run(args[2..].to_vec()));
        },
        "container_ps" => {
            match container::ps_a() {
                Ok(data) => {
                    print!("{}", serde_json::json!({
                        "code": "200",
                        "msg": "success",
                        "data": data
                    }));
                },
                Err(data) => {
                    print!("{}", serde_json::json!({
                        "code": "400",
                        "msg": data.to_string()
                    }));
                }
            }
        }
        "container_stop" => {
            result(container::stop(&args[2]));
        },
        "container_exists" => {
            result(container::exists(&args[2]));
        },
        "container_restart" => {
            result(container::restart(&args[2]));
        },
        "container_rm" => {
            result(container::rm(&args[2]));
        },
        "container_is_running" => {
            result(container::is_running(&args[2]));
        },
        "container_is_stop" => {
            result(container::is_stop(&args[2]));
        },
        "container_logs" => {
            result_string(container::logs(&args[2]));
        },
        "container_inspect" => {
            match container::inspect(&args[2]) {
                Ok(data) => {
                    print!("{}", serde_json::json!({
                        "code": "200",
                        "msg": "success",
                        "data": data
                    }));
                },
                Err(data) => {
                    print!("{}", serde_json::json!({
                        "code": "400",
                        "msg": data.to_string()
                    }));
                }
            };
        }
        "container_switch_gpu" => {
            result_string(container::switch_gpu(&args[2], args[3..].to_vec()));
        }
        _ => {
            print!("{}", serde_json::json!({
                "code": "400",
                "msg": "error",
                "data": "command not found"
            }));
            return Ok(());
        }
    }

    Ok(())
}

pub fn docker(mut vec: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    vec.insert(0, "docker");

    #[cfg(target_os = "windows")]
    let data = wei_run::command("wsl", vec)?;

    #[cfg(not(target_os = "windows"))]
    let data = wei_run::command("", vec)?;

    let error_vec = vec![
        "Cannot connect to the Docker daemon at",
        "requires exactly argument",
        "Usage:  docker [OPTIONS] COMMAND",
        "is not a docker command.",
        "Error response from daemon: No such image:",
        "Error response from daemon: pull access denied for",
        "Error response from daemon:",
    ];

    for item in error_vec {
        if data.contains(item) {
            return Err(data.into());
        }
    }

    Ok(data)
}

pub fn result_string(data: Result<String, Box<dyn std::error::Error>>) {
    match data {
        Ok(data) => {
            print!("{}", serde_json::json!({
                "code": "200",
                "msg": "success",
                "data": data
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": "400",
                "msg": data.to_string()
            }));
        }
    }
}

pub fn result_vec(data: Result<Vec<String>, Box<dyn std::error::Error>>) {
    match data {
        Ok(data) => {
            print!("{}", serde_json::json!({
                "code": "200",
                "msg": "success",
                "data": data
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": "400",
                "msg": data.to_string(),
            }));
        }
    }
}

pub fn result(data: Result<(), Box<dyn std::error::Error>>) {
    match data {
        Ok(()) => {
            print!("{}", serde_json::json!({
                "code": "200",
                "msg": "success",
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": "400",
                "msg": data.to_string()
            }));
        }
    }
}