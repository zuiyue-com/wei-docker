pub fn pull(name: &str, url: &str) -> Result<String,Box<dyn std::error::Error>> {
    let vec = vec!["wei-docker-linux", name, url];

    #[cfg(target_os = "windows")]
    let data = wei_run::command("wsl", vec)?;

    #[cfg(not(target_os = "windows"))]
    let data = wei_run::command("", vec)?;

    Ok(data)
}

pub fn progress(image_name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let image_name = if image_name.contains(":") {
        format!("{}", image_name)
    } else {
        format!("{}:latest", image_name)
    };

    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

    let image_name_encoded = utf8_percent_encode(&image_name, NON_ALPHANUMERIC).to_string();
    let file_path = format!("/root/.wei/docker/{}.json", image_name_encoded);

    let vec = vec!["cat", file_path.as_str()];

    #[cfg(target_os = "windows")]
    let data = wei_run::command("wsl", vec)?;

    #[cfg(not(target_os = "windows"))]
    let data = wei_run::command("", vec)?;

    let data = match serde_json::from_str::<serde_json::Value>(&data) {
        Ok(data) => data,
        Err(_) => serde_json::json!({})
    };

    Ok(data)
}

pub fn rmi(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = super::docker(vec!["rmi", name])?;

    Ok(data)
}

pub fn list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let data = super::docker(vec!["image", "ls", "--format", "{{.Repository}}:{{.Tag}}"])?;

    let mut vec = Vec::new();
    for item in data.split("\n") {
        if !item.is_empty() {
            vec.push(item.to_string());
        }
    }

    Ok(vec)
}


pub fn list_full() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let data = super::docker(
        vec!["images", "--format", 
        r#"
        {"Repository": "{{.Repository}}", 
        "Tag": "{{.Tag}}", 
        "ImageID": "{{.ID}}", 
        "CreatedAt": "{{.CreatedAt}}", 
        "Size": "{{.Size}}"}|||"#]
    )?;

    let mut value:serde_json::Value = serde_json::json!([]);
    for item in data.split("|||") {
        let item = item.trim();
        if !item.is_empty() {
            let c: serde_json::Value = serde_json::from_str(item)?;
            
            if let Some(array) = value.as_array_mut() {
                // 向数组中添加新元素
                array.push(c);
            }
        }
    }

    Ok(value)
}

pub fn exists(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = list()?;

    for item in data {
        if item == name {
            return Ok("image exists".into());
        }
    }

    Err("not exists".into())
}
