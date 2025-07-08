//! Rust网络基础学习
//! # 服务端
//! 1. 监听 TCP 连接
//! 2. 读取请求
//! 3. 发送响应
//! 
//! # 客户端
//! 1. 创建TCP连接
//! 2. 发生请求
//! 3. 读取响应

use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection is established.");

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    // 定义一个buffer由于接受数据
    let reader = BufReader::new(&mut stream);
    let request_body: Vec<_> = reader.lines().map(|result| {
        result.unwrap()
    }).take_while(|line| {
        !line.is_empty()
    }).collect();
    println!("Request body: {:#?}", request_body)
}
