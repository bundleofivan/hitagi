use std::net::{TcpListener, TcpStream};

pub struct Hitagi {
    port: String,
}

fn handle_client(stream: TcpStream) {
    println!("handling connection");
}

fn open_connection(port: String) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("localhost:{}", port))?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

impl Hitagi {
    pub fn init(port: String) -> Hitagi {
        open_connection(port.clone());
        Hitagi { port: port }
    }
}