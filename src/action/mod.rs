pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let param = vec!["/usr/bin/dockerd", "-H", "unix:///var/run/docker.sock", "-H", "tcp://0.0.0.0:2375"];
    
    #[cfg(target_os = "windows")] {}
    wei_run::command_async("wsl", param)?;

    #[cfg(not(target_os = "windows"))]
    wei_run::command_async("", param)?;
    
    Ok(())
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
