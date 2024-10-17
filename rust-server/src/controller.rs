use hyper::{Body, Method, Request, Response, StatusCode, header};
use crate::service;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let mut response_builder = Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS")
        .header(header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type");

    match (req.method(), req.uri().path()) {
        (&Method::OPTIONS, _) => {
            // 处理 CORS 预检请求
            Ok(response_builder.body(Body::empty()).unwrap())
        }
        (&Method::GET, "/api/hello") => {
            let result = service::get_hello();
            Ok(response_builder.body(Body::from(result)).unwrap())
        }
        (&Method::POST, "/api/echo") => {
            // 使用 hyper::body::to_bytes 将请求体转换为字节
            // 这里使用了 await 关键字，因为 to_bytes 返回一个 Future
            let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
            let result = service::post_echo(body_bytes);
            Ok(response_builder.body(Body::from(result)).unwrap())
        }
        _ => {
            //  404 Not Found
            Ok(response_builder
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    }
}
