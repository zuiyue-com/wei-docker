# wei docker

- [x] 安装参考文章：https://zhuanlan.zhihu.com/p/621142457
- [ ] 安装 docker
    - [ ] 中间有一次重开机的过程，这时候会导致程序断开，不需要理会他，也不需要上报任务状态，等机器重启完毕，程序会接收来自远程的任务继续做任务。任务服务器的任务状态应该有超时机制，超时后，任务服务器会继续等待任务完成。
    - [x] 制作 docker torrent  
    - [x] 下载 docker 安装包
    - [x] 安装流程不询问
    - [x] 测试空格目录，中文目录，特殊字符目录
    - [x] 安装 wsl_update.msi
    - [x] 安装 docker install
    - [x] 安装 ubuntu
    - [x] 安装 wsl ubuntu docker
        - [x] wsl --shutdown
        - [x] wsl --unregister wei-ubuntu
        - [x] wsl --import wei-ubuntu docker/wei-ubuntu docker/wei-ubuntu.tar --version 2
        - [x] wsl --set-default wei-ubuntu
    - [x] 安装 wsl nvidia cuda
    - [x] 安装 wsl nvidia cudnn
    - [x] 检测 wsl-update.msi
    - [x] 检测 wsl ubuntu
    - [x] 检测 wsl ubuntu docker
    - [x] 制作 ubuntu torrent
    - [ ] 测试英文系统安装
    
    
- [x] 开启 docker 服务
- [x] 关闭 docker 服务
- [ ] 镜像管理
    - [ ] 创建镜像
    - [ ] 镜像列表
    - [ ] 删除镜像
    - [ ] 镜像是否存在

- [ ] 容器管理
    - [ ] 创建容器
    - [ ] 启动容器
    - [ ] 容器列表
    - [ ] 停止容器
    - [ ] 删除容器
    - [ ] 容器是否存在
    - [ ] 容器是否运行
    - [ ] 容器是否停止
