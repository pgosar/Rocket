use socket2::{Domain, Socket, Type};
use std::io::prelude::*;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

pub struct Server {
    ip: String,
    port: u16,
    key: String,
}

impl Server {
    pub fn new(ip: String, port: u16, key: String) -> Server {
        Server {
            ip: ip,
            port: port,
            key: key
        }
    }
    pub fn run_server(&self) -> std::io::Result<()> {
        let sock = Socket::new(Domain::IPV6, Type::STREAM, None).unwrap();
        sock.set_only_v6(false)?;
        let address: SocketAddr = format!("[{}]:{}", self.ip, self.port).parse().unwrap();
        sock.bind(&address.into())?;
        sock.listen(128)?;
        let listener: TcpListener = sock.into();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    self.handle_client(stream);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Ok(())
    }
    
    pub fn handle_client(&self, mut stream: TcpStream) {
        println!("handling client");
        let mut buf = [0; 1024];
        stream.set_read_timeout(None).expect("set_read_timeout call failed");

        while let Ok(size) = stream.read(&mut buf) {
            if size == 0 {
                println!("size is 0");
                break;
            }
            println!("Server Received: {}", String::from_utf8_lossy(&buf[..size]));
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
        println!("client all done");
    }
}
