use socket2::{Domain, Socket, Type};
use std::io::prelude::*;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

struct Server {
    ip: String,
    port: u16,
    key: String,
}

pub fn run_server() -> std::io::Result<()> {
    let server = Server {
        ip: String::from("::1"),
        port: 8080,
        key: "1234567890".to_string(),
    };
    let sock = Socket::new(Domain::IPV6, Type::STREAM, None).unwrap();
    sock.set_only_v6(false)?;
    let address: SocketAddr = format!("[{}]:{}", server.ip, server.port).parse().unwrap();
    sock.bind(&address.into())?;
    sock.listen(128)?;
    let listener: TcpListener = sock.into();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}

pub fn handle_client(mut stream: TcpStream) {
    println!("handling client");
    let mut buf = [0; 1024];
    while let Ok(size) = stream.read(&mut buf) {
        if size == 0 {
            println!("size is 0");
            break;
        }
        match stream.write_all(&buf[0..size]) {
            Ok(_) => {
                println!("Server Sent: {}", String::from_utf8_lossy(&buf[..size]));
            }
            Err(_) => {
                println!(
                    "An error occurred while writing, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}
