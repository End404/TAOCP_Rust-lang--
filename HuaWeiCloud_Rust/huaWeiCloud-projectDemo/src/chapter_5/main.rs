// 第5章 从零实现简单Server

use crate::server::start;


mod server;
fn main() {
    start("127.0.0.1:8080".parse().unwrap());
}
