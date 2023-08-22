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



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let status = serde_json::json!({
    //     "code": "200",
    //     "status": "Ok"
    // });

    // println!("{}", status.get("code").expect("500").to_string());

    wei_env::bin_init("wei-docker");
    let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     // help();
    //     std::process::exit(1);
    // }
    let command = &args[1];

    match command.as_str() {
        "is_runing" => {
            println!("herea");
            is_runing().await?;
            println!("herea");
        },
        "uninstall" => {
            println!("Uninstalling...");
        },
        "check" => {

        },
        "api" => {
            // api().await?;
        },
        _ => {
            // help();
            std::process::exit(1);
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
                    "status" : "Failed to start Docker"
                })),
            }
        }
    }

    Ok(())
}


// pub async fn stop_docker(os_type: &Type) -> Result<(), Box<dyn std::error::Error>> {
//     let status = match os_type {
//         Type::Linux => {
//             AsyncCommand::new("sudo")
//                 .arg("systemctl")
//                 .arg("stop")
//                 .arg("docker")
//                 .status()
//                 .await?
//         }
//         Type::Macos => {
//             AsyncCommand::new("brew")
//                 .arg("services")
//                 .arg("stop")
//                 .arg("docker")
//                 .status()
//                 .await?
//         }
//         Type::Windows => {
//             AsyncCommand::new("powershell")
//                 .arg("Stop-Service")
//                 .arg("com.docker.service")
//                 .status()
//                 .await?
//         }
//         _ => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "Unsupported operating system",
//             )));
//         }
//     };

//     if !status.success() {
//         return Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             "Failed to stop Docker",
//         )));
//     }

//     Ok(())
// }


pub async fn image_create(image_name: &str) -> Result<(), bollard::errors::Error> {
    let docker = Docker::connect_with_local_defaults()?;

    let options = CreateImageOptions {
        from_image: image_name,
        ..Default::default()
    };

    let mut stream = docker.create_image(Some(options), None, None);

    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => println!("{:?}", output),
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    Ok(())
}

pub async fn image_exists(image_name: &str) -> Result<bool, Error> {
    // 创建 Docker 客户端
    let docker = Docker::connect_with_socket_defaults()?;

    // 创建镜像过滤器
    let mut filters = HashMap::new();
    filters.insert("reference".to_string(), vec![image_name.to_string()]);

    // 获取 Docker 镜像列表
    let options = Some(ListImagesOptions {
        filters: filters,
        ..Default::default()
    });
    let images = docker.list_images(options).await?;

    // 检查镜像是否存在
    let image_exists = !images.is_empty();

    Ok(image_exists)
}


pub async fn container_exists(container_name: &str) -> Result<bool, bollard::errors::Error> {
    let docker = Docker::connect_with_local_defaults()?;
    let mut filters = HashMap::new();
    filters.insert("name", vec![container_name]);

    let options = ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    };

    let containers = docker.list_containers(Some(options)).await?;
    Ok(!containers.is_empty())
}


pub async fn container_start(image_name: &str, container_name: &str, gpu_id: &str) -> io::Result<()> {
    let mut _gpu_device = format!("");
    let mut args = Vec::new();
    args.push("run");
    if !gpu_id.is_empty() {
        _gpu_device = format!("--gpus=\"device={}\"", gpu_id);
        args.push(_gpu_device.as_str());
    }
    args.push("--name");
    args.push(container_name);
    args.push("-d");
    args.push(image_name);

    let output = Command::new("docker").args(args).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("Failed to start container: {}", String::from_utf8_lossy(&output.stderr))))
    }
}


pub async fn container_resume(container_name: &str) -> io::Result<()> {
    let output = Command::new("docker").args(&["start",container_name]).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("Failed to resume container: {}", String::from_utf8_lossy(&output.stderr))))
    }
}

pub async fn container_stop(container_name: &str) -> io::Result<()> {
    let output = Command::new("docker").args(&["stop",container_name]).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("Failed to stop container: {}", String::from_utf8_lossy(&output.stderr))))
    }
}


// pub async fn container_rm(container_name: &str) -> io::Result<()> {
//     let output = Command::new("docker").args(&["rm",container_name]).output()?;

//     if output.status.success() {
//         Ok(())
//     } else {
//         Err(io::Error::new(io::ErrorKind::Other, format!("Failed to rm container: {}", String::from_utf8_lossy(&output.stderr))))
//     }
// }

