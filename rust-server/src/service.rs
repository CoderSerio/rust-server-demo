use hyper::body::Bytes;

pub fn get_hello() -> String {
    "Hello, World!".to_string()
}

// Bytes 是 hyper 提供的类型，代表一个字节序列
pub fn post_echo(body: Bytes) -> String {
    // from_utf8_lossy 会将无效的 UTF-8 序列替换为 U+FFFD REPLACEMENT CHARACTER
    String::from_utf8_lossy(&body).to_string()
}
