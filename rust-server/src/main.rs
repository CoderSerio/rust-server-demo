mod controller;
mod service;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

// #[tokio::main] 是一个属性宏，它会自动生成异步运行时，并运行 main
// 和 JS 的 async 类似； 而同样地，JS 中的 await 关键字，在 Rust 中是 xxx.await
#[tokio::main]
async fn main() {
    // 创建一个 SocketAddr，指定服务器监听的地址和端口
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // make_service_fn 创建一个服务工厂，用于为每个连接生成服务
    // 这里使用了闭包和 async/await 语法
    let make_svc = make_service_fn(|_conn| async {
        // service_fn 将一个异步函数转换为 hyper 的 Service
        // Infallible 表示这个 service 永远不会返回错误
        Ok::<_, Infallible>(service_fn(controller::handle_request))
    });

    // 创建并运行服务器
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    // 也就是上面提到的 await
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
