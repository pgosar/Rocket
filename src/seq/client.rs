use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub struct Client {
    server_uri: String,
    server_port: u16,
    server_path: String,
}

impl Client {
    pub fn new(uri: String) -> Client {
        let split_uri: std::vec::Vec<&str> = uri.split(':').collect();
        let port_path = String::from(split_uri[1]);
        let port_path_vec: std::vec::Vec<&str> = port_path.split('/').collect();
        let mut path = String::from("");
        if port_path_vec.len() > 1 {
            path = String::from(port_path_vec[1]);
        }
        let server_uri = String::from(split_uri[0]);
        let server_port = port_path_vec[0].parse::<u16>().unwrap();
        println!("{} {} {}", server_uri, server_port, path);
        Client {
            server_uri,
            server_port,
            server_path: path,
        }
    }

    /*pub fn handshake_http(stream: &mut TcpStream) -> &String {
        //dGhlIHNhbXBsZSBub25jZQ==
        //let my_addr: SocketAddr = stream.local_addr().unwrap();

        return format("GET {} HTTP/1.1\n\
            Host: {}\n\
            Upgrade: websocket\n\
            Connection: Upgrade\n\
            Sec-WebSocket-Key: {}\n\
            Origin: {}:{}\n\
            Sec-WebSocket-Version: 13", 

            self.server_uri, 


        );
    }*/

    pub fn run_client(&self, msg: String, repeats: i32) -> std::io::Result<()> {
        let address: String = format!("{}:{}", self.server_uri, self.server_port);
        println!("{}", address);
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                println!("Successfully connected to server in port {}", self.server_port);
                let byte_msg = msg.as_bytes();
                let mut data = [0 as u8; 1024];
                for _ in 0..repeats {
                    stream.write(byte_msg)?;
                    println!("Client Sent: {}", msg);
                    match stream.read(&mut data) {
                        Ok(_) => {
                            println!("Client Received: {}", from_utf8(&data).unwrap());
                        }
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
                // the size becomes 0 in server.rs when this call finishes because
                // the connection closes when the listener scope is gone
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
        Ok(())
    }
}
