use std::process::Command;

pub fn start() -> bool {
    let output = Command::new("powershell")
        .args(["/c", "wsl", "service", "docker", "start"])
        .output().unwrap();

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        info!("Command output: {}", s);

        if s.contains("OK") || s.contains("Starting Docker: docker") {
            return true;
        }
        return false;
    } 
    let s = String::from_utf8_lossy(&output.stderr);
    info!("Command failed, stderr: {}", s);
    false
}

pub fn stop() {
    let status = Command::new("powershell")
        .arg("/c")
        .arg("wsl")
        .arg("--shutdown")
        .status().unwrap();

    if !status.success() {
        info!("{}", "关闭Docker Desktop.exe运行失败");
    }
}

// fn docker_start_exe() -> String {
//     let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
//     let docker = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Docker Desktop");

//     if let Ok(docker) = docker {
//         let installation_path: Result<String, _> = docker.get_value("InstallLocation");

//         if let Ok(installation_path) = installation_path {
//             return format!("{}\\Docker Desktop.exe",installation_path);
//         }
//     }
//     "".to_string()
// }