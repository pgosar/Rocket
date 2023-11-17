use socket2::{Domain, Socket, Type};
use std::io::prelude::*;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use sha1::Digest;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

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

        Err(e) => {
          println!("Error: {}", e);
        }
      }
    }

    /*fn verify_structure(std::vec::Vec<&str>& lines) -> bool {
        // first line must be GET {} HTTP/1.1
        // you should be able to split each line by ": "
        // if you do that you have a pair of strings where the first is the key and the latter is the value
        // You want to see that Host, Upgrade, Connection, Sec-WebSocket-Key, Origin, Sec-WebSocket-Version are
        // all present and each only once
        // Upgrade: websocket, Connection: Upgrade, Sec-WebSocket-Version: 13
        let first_line: Vec<&str> = lines[0].split(" ").collect();
    }*/

    fn verify_client_handshake(&self, stream: &mut TcpStream) -> bool {
        let mut buf = [0; 1024];
        let size = stream.read(&mut buf).unwrap();
        let request = String::from_utf8_lossy(&buf[..size]);
        let lines: std::vec::Vec<&str> = request.split('\n').collect();
        let key: std::vec::Vec<&str> = lines[4].split(" ").collect();
        let combined = key[1].to_owned() + WEBSOCKET_PREFIX;
        let mut sha1 = sha1::Sha1::new();
        sha1.update(combined);
        let hash = sha1.finalize();
        let my_key: String = general_purpose::STANDARD.encode(&hash[..]);
        let response: String = format!("HTTP/1.1 101 Switching Protocols\n\
            Upgrade: websocket\n\
            Connection: Upgrade\n\
            Sec-WebSocket-Accept: {}",
            my_key
        );
        stream.write(response.as_bytes());
        /*
        HTTP/1.1 101 Switching Protocols
        Upgrade: websocket
        Connection: Upgrade
        Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=
        */
        true
    }


    pub fn handle_client(&self, mut stream: TcpStream) {
        println!("handling client");
        let mut buf = [0; 1024];
        //stream.set_read_timeout(None).expect("set_read_timeout call failed");

        let handshake_success: bool = self.verify_client_handshake(&mut stream);
        if handshake_success {
            while let Ok(size) = stream.read(&mut buf) {
                if size == 0 {
                    println!("size is 0");
                    break;
                }
                println!("Server Received: {}", String::from_utf8_lossy(&buf[..size]));
                match stream.write(&buf) {
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
        } else  {
            println!("Invalid client handshake");
        }

        println!("client all done");
    }
    println!("client all done");
  }
}
