pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")] {
        let param = vec!["service", "docker", "start"];
        wei_run::command_async("wsl", param)?;
    }
    
    #[cfg(not(target_os = "windows"))] {
        let param = vec!["/usr/bin/dockerd", "-H", "unix:///var/run/docker.sock", "-H", "tcp://0.0.0.0:2375"];
        wei_run::command_async("", param)?;
    }
    
    Ok(())
}


pub fn wsl_update() -> Result<(), Box<dyn std::error::Error>> {
    wei_run::command("wsl", 
        vec![
            "curl", 
            "-fSL", 
            "http://download.zuiyue.com/wsl/wei-docker-linux",
            "-o", 
            "/usr/bin/wei-docker-linux"
        ]
    )?;
    wei_run::command("wsl",
        vec![
            "chmod", 
            "+x", 
            "/usr/bin/wei-docker-linux"
        ]
    )?;
              
    Ok(())
}

pub fn is_autorun() -> Result<String, Box<dyn std::error::Error>> {
    let data = wei_env::read(
        &format!("{}docker-autorun.dat", wei_env::home_dir()?),
        "autorun"
    )?;

    Ok(data)
}

pub fn docker_autorun() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::write(
        &format!("{}docker-autorun.dat", wei_env::home_dir()?), 
        "autorun", "1"
    )?;

    Ok(())
}

pub fn docker_unautorun() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::write(
        &format!("{}docker-autorun.dat", wei_env::home_dir()?), 
        "autorun", "0"
    )?;

    Ok(())
}

pub fn is_started() {
    let data = match super::docker(vec!["images"]) {
        Ok(data) => data,
        Err(_) => {
            print!("{}", serde_json::json!({
                "code": 200,
                "message": "success",
                "is_start": "0"
            }));
            return;
        }
    };
    let mut is_start = "1";
    if data.contains("REPOSITORY") {
        is_start = "1";
    }
    
    print!("{}", serde_json::json!({
        "code": 200,
        "message": "success",
        "is_start": is_start
    }));
}

pub fn is_installed() {
    let data = wei_docker_install::check();

    let mut is_installed = false;

    match data["ubuntu"].as_bool() {
        Some(data) => {
            if data {
                is_installed = true;
            }
        },
        None => {}
    }

    print!("{}",serde_json::json!({
        "code": 200,
        "message": "success",
        "is_installed": is_installed
    }));
}

pub fn api() -> Result<(), Box<dyn std::error::Error>> {

    #[cfg(target_os = "windows")]
    let ip = wei_run::command("wsl", vec!["hostname", "-I"])?;

    #[cfg(not(target_os = "windows"))]
    let ip = wei_run::command("", vec!["hostname", "-I"])?;

    let ip: Vec<&str> = ip.split(" ").collect();
    let ip = ip[0];
    let url = format!("http://{}:2375", ip);

    // let res;

    // let method = method.to_uppercase();
    // let method = method.as_str();
    // match method {
    //     "GET" => {
    //         res = match ureq::get(&url).call() {
    //             Ok(res) => res,
    //             Err(err) => {
    //                 print!("{}", serde_json::json!({
    //                     "code": 400,
    //                     "message": err.to_string()
    //                 }));
    //                 return Ok(());
    //             }
    //         };
    //     },
    //     "POST" => {
    //         res = match ureq::post(&url)
    //         .set("Content-Type", "application/json")
    //         .send_string(data) {
    //             Ok(res) => res,
    //             Err(err) => {
    //                 print!("{}", serde_json::json!({
    //                     "code": 400,
    //                     "message": err.to_string()
    //                 }));
    //                 return Ok(());
    //             }
    //         };
    //     },
    //     "DELETE" => {
    //         res = match ureq::delete(&url).call() {
    //             Ok(res) => res,
    //             Err(err) => {
    //                 print!("{}", serde_json::json!({
    //                     "code": 400,
    //                     "message": err.to_string()
    //                 }));
    //                 return Ok(());
    //             }
    //         };
    //     },
    //     _ => {
    //         print!("{}", serde_json::json!({
    //             "code": 400,
    //             "message": "error",
    //             "data": format!("METHOD: {} not found", method)
    //         }));
    //         return Ok(());
    //     }
    // }

    // let status = res.status();
    // let data = res.into_string()?;
    // let data: serde_json::Value = match serde_json::from_str(&data) {
    //     Ok(data) => data,
    //     Err(_) => {
    //         print!("{}", serde_json::json!({
    //             "code": 200,
    //             "message": "success",
    //             "status": status
    //         }));
    //         return Ok(());
    //     }
    // };

    print!("{}", serde_json::json!({
        "code": 200,
        "message": "success",
        "data": url
    }));

    Ok(())
}

// pub fn reinstall() -> Result<(), Box<dyn std::error::Error>> {
//     // 

//     Ok(())
// }

pub fn stop() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    wei_run::command("wsl", vec!["--shutdown"])?;

    #[cfg(not(target_os = "windows"))]
    wei_run::command("pkill", vec!["dockerd"])?;

    Ok(())
}

pub fn one_click() -> Result<(), Box<dyn std::error::Error>> {
    // 一键安装的记录
    wei_env::write(
        &format!("{}docker-one-click.dat", wei_env::home_dir()?), 
        "install", "1"
    )?;

    // 设置开机启动
    // autorun()?;

    // 下载 docker
    wei_run::run("wei-docker", vec!["download"])?;

    // 检测 docker 完整性
    loop {
        let data = wei_run::run("wei-docker", vec!["download_check"])?;
    
        let data: serde_json::Value = match serde_json::from_str(&data) {
            Ok(data) => data,
            Err(err) => {
                info!("error:{}, data:{}", err, data);
                serde_json::json!({})
            }
        };
        
        match data["code"].as_i64() {
            Some(code) => {
                if code == 200 && data["data"].is_object() {
                    let complete_length = data["data"]["completed_length"].as_str().unwrap_or("0");
                    let total_length = data["data"]["total_length"].as_str().unwrap_or("100");

                    if complete_length == total_length {
                        break;
                    }
                }
            },
            None => {}
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // 安装 docker
    wei_run::run("wei-docker", vec!["install"])?;

    // 重启
    // 开机启动客户端 
    // 检测到一键安装的设置
    // 删除一键安装的设置
    // 安装 ubuntu

    Ok(())
}


use winreg::enums::*;
use winreg::RegKey;
use std::env;
pub fn _autorun() -> Result<(), Box<dyn std::error::Error>> {
    info!("autorun");
    let path = env::current_exe()?;
    let path_str = path.to_str().ok_or("Invalid path")?;
    let path_str = path_str.replace("\\", "/");
    let path_str = path_str.replace("data/wei-docker.exe", "wei.exe");

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")?;

    key.set_value("Wei", &path_str)?;
    Ok(())
}


pub fn _unautorun() -> Result<(), Box<dyn std::error::Error>> {
    info!("unautorun");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE)?;
    
    key.delete_value("Wei")?;
    Ok(())
}
