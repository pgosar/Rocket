use socket2::{Domain, Socket, Type};
use std::io::prelude::*;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

const WEBSOCKET_PREFIX: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub struct Server {
    ip: String,
    port: u16,
    key: String,
    listener: TcpListener,
}


impl Server {
    pub fn new(ip: String, port: u16, key: String) -> Server {
        let sock = Socket::new(Domain::IPV6, Type::STREAM, None).unwrap();
        sock.set_only_v6(false).unwrap();
        let address: SocketAddr = format!("[{}]:{}", ip, port).parse().unwrap();
        sock.bind(&address.into()).unwrap();
        sock.listen(128).unwrap();
        let listener: TcpListener = sock.into();
        Server {
            ip,
            port,
            key,
            listener,
        }
    }

    pub fn run_server(&self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
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
        //stream.set_read_timeout(None).expect("set_read_timeout call failed");

        //let size = stream.read(&mut buf).unwrap();
        //let request = String::from_utf8_lossy(&buf[..size]);
        //let lines: std::vec::Vec<&str> = request.split('\n').collect();


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
