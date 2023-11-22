use crate::server::server::WEBSOCKET_PREFIX;
use crate::utils::logging::*;
use base64::engine::general_purpose;
use base64::Engine;
use sha1::Digest;
use std::net::SocketAddr;
use std::vec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};


pub struct ConcurrentServer {
  ip: String,
  port: u16,
  key: String,
  listener: TcpListener,
  server_log: Arc<Mutex<Logger>>,
}

async fn create_listener(ip: String, port: u16) -> TcpListener {
  let address: SocketAddr = format!("[{}]:{}", ip, port).parse().unwrap();
  let listener: TcpListener = TcpListener::bind(address).await.unwrap();
  listener
}

impl ConcurrentServer {
  pub async fn new(ip: String, port: u16, key: String) -> ConcurrentServer {
    ConcurrentServer {
      ip: ip.clone(),
      port,
      key,
      listener: create_listener(ip, port).await,
      server_log: Arc::new(Mutex::new(Logger::new())),
    }
  }

  pub async fn run_server(&mut self) -> std::io::Result<()> {
    println!("Server running on {}:{}", self.ip, self.port);
    let server_log = &mut self.server_log;
    loop {
      let log_copy = Arc::clone(server_log);
      let (stream, addr) = self.listener.accept().await?;
      println!("New client: {}", addr);
      tokio::spawn(async move {
        Self::handle_client(&log_copy, stream).await;
      });
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

  async fn verify_client_handshake(stream: &mut TcpStream) -> bool {
    let mut buf = [0; 1024];
    let size = stream.read(&mut buf).await.unwrap();
    let request = String::from_utf8_lossy(&buf[..size]);
    let lines: std::vec::Vec<&str> = request.split('\n').collect();
    let key: std::vec::Vec<&str> = lines[4].split(" ").collect();
    let combined = key[1].to_owned() + WEBSOCKET_PREFIX;
    let mut sha1 = sha1::Sha1::new();
    sha1.update(combined);
    let hash = sha1.finalize();
    let my_key: String = general_purpose::STANDARD.encode(&hash[..]);
    let response: String = format!(
      "HTTP/1.1 101 Switching Protocols\n\
            Upgrade: websocket\n\
            Connection: Upgrade\n\
            Sec-WebSocket-Accept: {}",
      my_key
    );
    stream.write(response.as_bytes()).await.unwrap();
    true
  }

  async fn read_message(server_log: &Arc<Mutex<Logger>>, buf: &mut Vec<u8>, stream: &mut TcpStream) -> bool {
    let size = stream.read(buf).await.unwrap();
    if size == 0 {
      println!("size is 0");
      return false;
    }
    let msg: String = format!("Server Read: {}", String::from_utf8_lossy(&buf[..]));
    let m: Message = Message::new(msg.clone(), ErrorLevel::INFO);
    let mut logger = server_log.lock().unwrap();
    logger.log(m);
    true
  }

  async fn write_message(server_log: &Arc<Mutex<Logger>>, buf: &mut Vec<u8>, stream: &mut TcpStream) -> bool {
    match stream.write(&buf).await {
      Ok(_) => {
        let msg: String = format!("Server Write: {}", String::from_utf8_lossy(&buf[..]));
        let m: Message = Message::new(msg.clone(), ErrorLevel::INFO);
        let mut logger = server_log.lock().unwrap();
        logger.log(m);
        true
      }
      Err(_) => {
        println!(
          "An error occurred while writing, terminating connection with {}",
          stream.peer_addr().unwrap()
        );
        stream.shutdown().await.unwrap();
        false
      }
    }
  }

  pub async fn handle_client(server_log: &Arc<Mutex<Logger>>, mut stream: TcpStream) {
    println!("handling client");
    let mut buf: Vec<u8> = vec![0; 1024];
    let handshake_success: bool = Self::verify_client_handshake(&mut stream).await;
    if handshake_success {
      while Self::read_message(server_log, &mut buf, &mut stream).await {
        if !Self::write_message(server_log, &mut buf, &mut stream).await {
          break;
        }
      }
    } else {
      println!("Invalid client handshake");
    }
    println!("client all done");
    let logger = server_log.lock().unwrap();
    logger.print_log();
  }
}
