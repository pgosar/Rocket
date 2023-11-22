use crate::utils::logging::*;
use crate::utils::utils::{Opts, sec_websocket_key};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::vec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::collections::HashMap;


pub struct ConcurrentServer {
  ip: String,
  port: u16,
  key: String,
  listener: TcpListener,
  server_log: Arc<Mutex<Logger>>,
  opts: Opts,
}

async fn create_listener(ip: String, port: u16) -> TcpListener {
  let address: SocketAddr = format!("[{}]:{}", ip, port).parse().unwrap();
  let listener: TcpListener = TcpListener::bind(address).await.unwrap();
  listener
}

impl ConcurrentServer {
  pub async fn new(ip: String, port: u16, key: String, opts: Opts) -> ConcurrentServer {
    ConcurrentServer {
      ip: ip.clone(),
      port,
      key,
      listener: create_listener(ip, port).await,
      server_log: Arc::new(Mutex::new(Logger::new())),
      opts,
    }
  }

  pub async fn run_server(&mut self) -> std::io::Result<()> {
    if *self.opts.debug() {
      println!("Server running on {}:{}", self.ip, self.port);
    }
    let server_log = &mut self.server_log;
    loop {
      let log_copy = Arc::clone(server_log);
      let (stream, addr) = self.listener.accept().await?;
      if *self.opts.debug() {
        println!("New client: {}", addr);
      }
      let debug = (self.opts.debug()).clone();
      tokio::spawn(async move {
        Self::handle_client(&log_copy, stream, debug).await;
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
    let first_line: vec::Vec<&str> = lines[0].split(' ').collect();
    let last_word = format!(r"{}", first_line[2]);
    if first_line.len() != 3 || first_line[0] != "GET" || 
       !first_line[1].starts_with('/') || last_word.trim() != r"HTTP/1.1" {
      return false;
    }
    let mut m: HashMap<String, String> = HashMap::new();
    for line in lines[1..].iter() {
      let split_line: Vec<&str> = (line.to_owned()).split(": ").collect();
      if split_line.len() == 2 {
        m.insert(String::from(split_line[0]), String::from(split_line[1]));
      }
    }
    let host = m.get("Host").unwrap().to_owned();
    let upgrade = m.get("Upgrade").unwrap().to_owned();
    let connection = m.get("Connection").unwrap().to_owned();
    let key = m.get("Sec-WebSocket-Key").unwrap().to_owned();
    let version = m.get("Sec-WebSocket-Version").unwrap().to_owned();
    let origin = m.get("Origin").unwrap().to_owned();

    if upgrade.trim() != "websocket" || connection.trim() != "Upgrade" || version.trim() != "13" {
      return false;
    }
    let my_key = sec_websocket_key(key);
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

  

  pub async fn read_message(
    server_log: &Arc<Mutex<Logger>>,
    buf: &mut Vec<u8>,
    stream: &mut TcpStream,
    debug: bool,
  ) -> bool {
    let size = stream.read(buf).await.unwrap();
    if size == 0 {
      if debug {
        println!("size is 0");
      }
      return false;
    }
    let msg: String = format!("Server Read: {}", String::from_utf8_lossy(&buf[..]));
    let m: Message = Message::new(msg.clone(), ErrorLevel::INFO);
    let mut logger = server_log.lock().unwrap();
    logger.log(m);
    true
  }

  pub async fn write_message(
    server_log: &Arc<Mutex<Logger>>,
    buf: &mut Vec<u8>,
    stream: &mut TcpStream,
  ) -> bool {
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

  pub async fn handle_client(server_log: &Arc<Mutex<Logger>>, mut stream: TcpStream, debug: bool) {
    let mut buf: Vec<u8> = vec![0; 1024];
    let handshake_success: bool = Self::verify_client_handshake(&mut stream).await;
    if handshake_success {
      while Self::read_message(server_log, &mut buf, &mut stream, debug).await {
        if !Self::write_message(server_log, &mut buf, &mut stream).await {
          break;
        }
      }
    } else {
      if debug {
        println!("Invalid client handshake");
      }
    }
    if debug {
      println!("client all done");
    }
    let logger = server_log.lock().unwrap();
    logger.print_log();
  }
}
