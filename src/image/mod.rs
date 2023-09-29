
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
