
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

