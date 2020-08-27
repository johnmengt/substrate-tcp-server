// TcpListener 是一个socket server的结构
// TcpStream 是一个Local socket 和remote socket之间的流
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};              //为了处理错误,用到io库
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8899").expect("Unable to bind to socket");

    println!("Listening on port {}", 8899);    //监听端口

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                thread::spawn(|| { handle_client(stream); });
            }
            Err(_) => {
                continue;
            },
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("connection accepted");

    let mut buffer: [u8; 1024] = [0; 1024];       //创建一个buffer

    loop {
        match stream.read(&mut buffer) {       //从buffer里读内容
            Ok(n) => {
                if n == 0 {
                    break;
                }
                if let Err(_) = stream.write(&buffer[..n]) {        // 用于打印从client接收的结果
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    println!("disconnected")
}
