// docker run -it --gpus all --name my_docker -e NVIDIA_DRIVER_CAPABILITIES=compute,utility -e NVIDIA_VISIBLE_DEVICES=all ubuntu:latset
use crate::image;
pub mod types;

use types::{Container, ContainerStat};//, ContainerInspect};

/// 容器运行
pub fn run(args: Vec<String>) -> Result<String,Box<dyn std::error::Error>> {
    let mut param = Vec::new();

    param.push("run");
    
    for i in 0..args.len() {
        param.push(args[i].as_str());
    }

    let data = super::docker(param)?;

    Ok(data)
}


// 容器列表
// 容器的统计信息(硬盘大小ps -s -a，内存大小，cpu使用率，gpu使用率) 
pub fn ps_a() -> Result<Vec<Container>,Box<dyn std::error::Error>> {
    let data = super::docker(vec!["ps", "-a", "-s", "--format", 
r#"{
    "id": "{{.ID}}",
    "names": "{{.Names}}",
    "image": "{{.Image}}",
    "created": "{{.CreatedAt}}",
    "status": "{{.Status}}",
    "ports": "{{.Ports}}",
    "size": "{{.Size}}"
}|||"#])?;


    let mut vec = Vec::new();
    for item in data.split("|||") {
        let item = item.trim();
        if !item.is_empty() {
            let c: Container = serde_json::from_str(item)?;
            
            vec.push(c);
        }
    }

    Ok(vec)
}

// 停止容器 stop
pub fn stop(name: &str) -> Result<(),Box<dyn std::error::Error>> {
    super::docker(vec!["stop", name])?;

    Ok(())
}

pub fn stop_all() -> Result<(), Box<dyn std::error::Error>> {
    let data = ps_a()?;

    for item in data {
        stop(item.names.as_str())?;
    }

    Ok(())
}

// 恢复/重启容器 restart
pub fn restart(name: &str) -> Result<(),Box<dyn std::error::Error>> {
    super::docker(vec!["restart", name])?;

    Ok(())
}

// 删除容器 stop && rm
pub fn rm(name: &str) -> Result<(),Box<dyn std::error::Error>> {
    stop(name)?;
    super::docker(vec!["rm", name])?;

    Ok(())
}

// 容器是否存在 exists
pub fn exists(name: &str) -> Result<(),Box<dyn std::error::Error>> {
    let data = ps_a()?;
    
    for item in data {
        if item.names == name {
            return Ok(());
        }
    }

    Err("container not exists".into())
}

// 容器是否运行 is_running
pub fn is_running(name: &str) -> Result<(),Box<dyn std::error::Error>> {
    let data = ps_a()?;

    for item in data {
        if item.names == name && item.status.contains("Up") {
            return Ok(());
        }
    }

    Err("container not running".into())
}

// 容器是否停止 is_stop
pub fn is_stop(name: &str) -> Result<(),Box<dyn std::error::Error>> {
    let data = ps_a()?;

    for item in data {
        if item.names == name && item.status.contains("Exited") {
            return Ok(());
        }
    }

    Err("container not stop".into())
}

// 容器日志 logs
pub fn logs(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = super::docker(vec!["logs", name])?;

    Ok(data)    
}

pub fn inspect(name: &str) -> Result<ContainerStat, Box<dyn std::error::Error>> {
    // let data = super::docker(vec!["inspect", name])?;

    // let data = data.trim();
    // let data = data.trim_start_matches("[");
    // let data = data.trim_end_matches("]");
    // let data = data.trim();

    // let data: ContainerInspect = serde_json::from_str(data)?;
    // println!("{}", data.host_config.DeviceRequests.unwrap()[0].DeviceIDs[0]);

    let gpu = super::docker(vec!["inspect", "-f", "{{range .HostConfig.DeviceRequests}}{{range .DeviceIDs}}{{.}}{{end}}{{end}}", name])?;
    let gpu = gpu.trim();

    let ip = super::docker(vec!["inspect", "-f", "{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}", name])?;
    let ip = ip.trim();

    let stat = super::docker(vec!["stats", "--no-stream", "--format","{{.ID}} {{.Name}} {{.CPUPerc}} {{.MemUsage}} {{.MemPerc}} {{.NetIO}} {{.BlockIO}} {{.PIDs}}", name])?;
    let parts: Vec<&str> = stat.split_whitespace().collect();

    let data: ContainerStat = ContainerStat {
        id: parts[0].to_string(),
        name: parts[1].to_string(),
        gpu: gpu.to_string(),
        ip_address: ip.to_string(),
        cpu_percentage: parts[2].to_string(),
        mem_usage: parts[3].to_string(),
        mem_limit: parts[5].to_string(),
        mem_percentage: parts[6].to_string(),
        net_io: parts[7].to_string(),
        block_io: parts[9].to_string(),
        pids: parts[10].to_string(),
    };

    Ok(data)
}

pub fn switch_gpu(name: &str, args: Vec<String>) -> Result<String,Box<dyn std::error::Error>> {
    stop(name)?;
    super::docker(vec!["commit", name, name])?;
    super::docker(vec!["rm", name])?;

    run(args)
}

// pub async fn container_start(image_name: &str, container_name: &str, gpu_id: &str) -> io::Result<()> {
//     let mut _gpu_device = format!("");
//     let mut args = Vec::new();
//     args.push("run");
//     if !gpu_id.is_empty() {
//         _gpu_device = format!("--gpus=\"device={}\"", gpu_id);
//         args.push(_gpu_device.as_str());
//     }

pub fn fix_nvidia(image_name: &str, container_name: &str) -> Result<(),Box<dyn std::error::Error>> {
    // 使用非gpu模式运行 容器，tail -f /dev/null
    super::docker(vec!["run", "-d", "--name", container_name, image_name, "tail", "-f", "/dev/null"])?;

    // docker exec -it 容器名 rm /usr/lib/x86_64-linux-gnu/libnvidia-ml.so.1
    super::docker(vec!["exec", "-i", container_name, "rm", "/usr/lib/x86_64-linux-gnu/libnvidia-ml.so.1"])?;
    super::docker(vec!["exec", "-i", container_name, "rm", "/usr/lib/x86_64-linux-gnu/libcuda.so.1"])?;
    super::docker(vec!["exec", "-i", container_name, "rm", "/usr/lib/x86_64-linux-gnu/libcudadebugger.so.1"])?;

    super::docker(vec!["stop", container_name])?;
    super::docker(vec!["commit", container_name, container_name])?;
    super::docker(vec!["rm", container_name])?;

    Ok(())
}

// pub fn run_code(code: &str, image_name: &str, container_name: &str, gpu_id: &str) -> Result<(),Box<dyn std::error::Error>> {
    

//     Ok(())
// }

pub fn exec_code(container_name: &str, code: &str) -> Result<String,Box<dyn std::error::Error>> {
    let code = base64::decode(code)?;

    let code = match String::from_utf8(code.clone()) {
        Ok(data) => data,
        Err(err) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": err.to_string()
            }));
            std::process::exit(0);
        }
    };

    // 把code写入文件
    let file_name = format!("./{}_code.sh", container_name);
    std::fs::write(file_name.as_str(), code.as_str())?;

    // 再把文件复制到容器里面
    super::docker(vec!["cp", file_name.as_str(), format!("{}:/code.sh", container_name).as_str()])?;
    // 删除本地文件
    match std::fs::remove_file(file_name.as_str()) {
        Ok(_) => {},
        Err(_) => {}
    };
    // 最后exec 执行脚本
    let data = super::docker(vec!["exec", "-i", container_name, "sh", "/code.sh"])?;
    // 删除容器的脚本
    super::docker(vec!["exec", "-i", container_name, "rm", "/code.sh"])?;

    Ok(data)
}

pub fn one_click(args: Vec<String>) -> Result<String,Box<dyn std::error::Error>> {
    let url = args[args.len() - 1].clone();
    let last = args.len() - 1;
    let image_name = args[2].clone();
    let code = match base64::decode(args[3].clone()) {
        Ok(data) => data,
        Err(err) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": err.to_string()
            }));
            std::process::exit(0);
        }
    };

    let url = base64::encode(url);
    match image::pull(&image_name, &url) {
        Ok(_) => {},
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": data.to_string()
            }));
            std::process::exit(0);
        }
    };

    image::clear_none()?;

    run(args[4..last].to_vec())
}