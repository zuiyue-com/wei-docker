pub fn pull(name: &str) -> Result<String,Box<dyn std::error::Error>> {
    let data = super::docker(vec!["pull", name])?;

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

pub fn exists(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = list()?;

    for item in data {
        if item == name {
            return Ok(());
        }
    }

    Err("not exists".into())
}
