#[macro_use]
extern crate wei_log;

mod action;
mod image;
mod container;

use std::env;

// use wei_download::DownloadMethod::QBitTorrent;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    wei_env::bin_init("wei-docker");
    let args: Vec<String> = env::args().collect();

    let mut command = "".to_string();
    if args.len() > 1 {
        command = args[1].clone();
    }

    match command.as_str() {
        "one_click" => {
            result(action::one_click());
        }
        "autorun" => {
            result(action::autorun());
        }
        "unautorun" => {
            result(action::unautorun());
        }
        "download" => {
            info!("Downloading...");
            println!("{}", 
                wei_run::run("wei-download", 
                vec![
                    "add", 
                    "http://download.zuiyue.com/windows/torrent/docker.torrent",
                    &std::env::current_dir()?.display().to_string()
                    ])?
            );
        }
        "download_check" => {
            println!("{}", 
                wei_run::run("wei-download", 
                vec![
                    "list", 
                    "docker",
                    ])?
            );
        }
        "install" => {
            info!("Installing...");
            print!("{}",serde_json::json!({
                "code": 200,
                "message": "success"
            }));
            wei_docker_install::install();
        },
        "uninstall" => {
            info!("Uninstalling...");
            wei_docker_install::uninstall();
            print!("{}",serde_json::json!({
                "code": 200,
                "message": "success"
            }));
        },
        "check" => {
            print!("{}",serde_json::json!({
                "code": 200,
                "message": "success",
                "progress": wei_docker_install::check()
            }));
        },
        "is_started" => {
            action::is_started();
        }
        "is_installed" => {
            action::is_installed();
        },
        "docker_service_auto_start" => {
            result(action::docker_autorun());
        },
        "docker_service_auto_stop" => {
            result(action::docker_unautorun());
        },
        "start" => {
            info!("Starting...");
            action::start()?;
            print!("{}",serde_json::json!({
                "code": 200,
                "message": "success"
            }));
        },
        "stop" => {
            info!("Stoping...");
            action::stop()?;
            print!("{}",serde_json::json!({
                "code": 200,
                "message": "success"
            }));
        }
        "api" => {
            action::api()?;
        }
        "image_pull" => {
            let mut url = "".to_string();
            if args.len() > 3 {
                url = base64::encode(args[3].clone());
            }
            print!("{}", image::pull(&args[2], &url)?);
        },
        "image_progress" => {
            print!("{}", serde_json::json!({
                "code": 200,
                "message": "success",
                "data": image::progress(&args[2])?
            }));
        }
        "image_rmi" => {
            result_string(image::rmi(&args[2]));
        },
        "image_list" => {
            result_vec(image::list());
        },
        "image_list_full" => {
            result_value(image::list_full());
        },
        "image_exists" => {
            result_string(image::exists(&args[2]));
        },
        "container_run" => {
            let url_progress = args[args.len() - 1].clone();

            if url_progress.contains("http") {
                result_string(container::run(args[2..args.len()-1].to_vec()));
                return Ok(());
            }

            result_string(container::run(args[2..].to_vec()));
        },
        "image_pull_container_run" => {
            // 如果倒数第二个参数包含 http 则认为是 url
            let url = args[args.len() - 1].clone();
            let last = args.len() - 1;

            info!("args: {:?}", args[3..last].to_vec());

            let url = base64::encode(url);
            image::pull(&args[2], &url)?;
            result_string(container::run(args[3..last].to_vec()));
        },
        "container_ps" => {
            match container::ps_a() {
                Ok(data) => {
                    print!("{}", serde_json::json!({
                        "code": 200,
                        "message": "success",
                        "data": data
                    }));
                },
                Err(data) => {
                    print!("{}", serde_json::json!({
                        "code": 400,
                        "message": data.to_string()
                    }));
                }
            }
        }
        "container_stop" => {
            result(container::stop(&args[2]));
        },
        "container_stop_all" => {
            result(container::stop_all());
        }
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
                        "code": 200,
                        "message": "success",
                        "data": data
                    }));
                },
                Err(data) => {
                    print!("{}", serde_json::json!({
                        "code": 400,
                        "message": data.to_string()
                    }));
                }
            };
        }
        "container_switch_gpu" => {
            let url_progress = args[args.len() - 1].clone();

            if url_progress.contains("http") {
                result_string(container::switch_gpu(&args[2], args[3..args.len()-1].to_vec()));
                return Ok(());
            }

            result_string(container::switch_gpu(&args[2], args[3..].to_vec()));
        }
        "container_fix_nvidia" => {
            // rm /usr/lib/x86_64-linux-gnu/libnvidia-ml.so.1
            // rm /usr/lib/x86_64-linux-gnu/libcuda.so.1
            // rm /usr/lib/x86_64-linux-gnu/libcudadebugger.so.1
        }
        // "container_install_"
        "wsl_update" => {
            result(action::wsl_update());
        }
        _ => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": "error",
                "data": format!("{} not found", command)
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

    // 如果长度等于65个字母，则返回正确
    if data.len() == 65 {
        return Ok(data);
    }

    let error_vec = vec![
        "Cannot connect to the Docker daemon at",
        "requires exactly argument",
        "Usage:  docker [OPTIONS] COMMAND",
        "is not a docker command.",
        "Error response from daemon: No such image:",
        "Error response from daemon: pull access denied for",
        "Error response from daemon:",
        "requires at least 1 argument.",
        "error:"
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
                "code": 200,
                "message": "success",
                "data": data
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": data.to_string()
            }));
        }
    }
}

pub fn result_value(data: Result<serde_json::Value, Box<dyn std::error::Error>>) {
    match data {
        Ok(data) => {
            print!("{}", serde_json::json!({
                "code": 200,
                "message": "success",
                "data": data
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": data.to_string(),
            }));
        }
    }
}

pub fn result_vec(data: Result<Vec<String>, Box<dyn std::error::Error>>) {
    match data {
        Ok(data) => {
            print!("{}", serde_json::json!({
                "code": 200,
                "message": "success",
                "data": data
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": data.to_string(),
            }));
        }
    }
}

pub fn result(data: Result<(), Box<dyn std::error::Error>>) {
    match data {
        Ok(()) => {
            print!("{}", serde_json::json!({
                "code": 200,
                "message": "success",
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": data.to_string()
            }));
        }
    }
}