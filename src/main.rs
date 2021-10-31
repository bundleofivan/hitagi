use std::net::{TcpListener, TcpStream};
mod hitagi;
use crate::hitagi::Hitagi;

fn handle_client(stream: TcpStream) {
    println!("hello world");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    Hitagi::init("123".to_string());

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
