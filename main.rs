use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8899").expect("Unable to bind to socket");

    println!("Listening on port {}", 8899);

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

    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                if let Err(_) = stream.write(&buffer[..n]) {
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
