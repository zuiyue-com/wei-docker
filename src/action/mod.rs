use std::process::Command;

pub fn start() -> bool {
    let docker_exe_path = docker_start_exe();
    if docker_exe_path == "" {
        return false;
    }
    let path = std::path::Path::new(&docker_exe_path);
    if !path.is_file() {
        info!("{}", "Docker Desktop.exe文件不存在");
        return false;
    }

    let status = Command::new("powershell")
        .arg("/c")
        .arg(&format!("& '{}'", docker_exe_path))
        .status().unwrap();

    if !status.success() {
        info!("{}", "Docker Desktop.exe运行失败");
        return false;
    }

    true
}

pub fn stop() {
    let status = Command::new("powershell")
        .arg("/c")
        .arg("taskkill")
        .arg("/IM")
        .arg("\"Docker Desktop.exe\"")
        .arg("/F")
        .status().unwrap();

    if !status.success() {
        info!("{}", "关闭Docker Desktop.exe运行失败");
    }
}

fn docker_start_exe() -> String {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let docker = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Docker Desktop");

    if let Ok(docker) = docker {
        let installation_path: Result<String, _> = docker.get_value("InstallLocation");

        if let Ok(installation_path) = installation_path {
            return format!("{}\\Docker Desktop.exe",installation_path);
        }
    }
    "".to_string()
}