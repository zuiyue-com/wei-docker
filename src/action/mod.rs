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
