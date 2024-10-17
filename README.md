# Rust Web Server Demo

这是一个简单的 Rust Web 服务器演示项目，配合一个基本的前端页面。

## 前提条件

- 安装 Rust 和 Cargo。
- 安装 Node.js（用于运行一个简单的前端服务器，当然如果你用 VSCode 插件提供的服务器之类的，也可以不用安装 Node.js）。

## 启动后端服务器

1. 进入 rust-server 目录：  
```shell
cd rust-server
```

3. 运行 Rust 服务器：   
```shell
cargo run   
```

服务器将在 http://localhost:3000 上运行。

## 启动前端服务器

1. 进入 frontend 目录：   
```shell
cd frontend  
```

3. 使用 Node.js 的 http-server 或类似工具启动一个简单的 HTTP 服务器。如果你没有安装 http-server，可以通过 npm 安装：   
```shell
npm install -g http-server   
```

5. 启动前端服务器：   
```shell
http-server -p 8080   
```

   前端页面将在 http://localhost:8080 上可用。

