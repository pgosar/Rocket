use crate::utils::logging::*;
use crate::utils::utils::{sec_websocket_key, Opts};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::vec;
// asynchronous enabled
use pollster::block_on;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct ConcurrentServer {
  ip: String,
  port: u16,
  key: String,
  listener: TcpListener,
  server_log: Arc<Mutex<Logger>>,
  debug: bool,
  seq: bool,
}

async fn create_listener(ip: String, port: u16, seq: bool) -> TcpListener {
  let address: SocketAddr = format!("[{}]:{}", ip, port).parse().unwrap();
  if seq {
    block_on(async {
      let listener = TcpListener::bind(&address).await.unwrap();
      listener
    })
  } else {
    TcpListener::bind(&address).await.unwrap()
  }
}

impl ConcurrentServer {
  pub async fn new(ip: String, port: u16, key: String, opts: Opts) -> ConcurrentServer {
    let seq = *opts.mode() == "s";
    ConcurrentServer {
      ip: ip.clone(),
      port,
      key,
      listener: create_listener(ip, port, seq).await,
      server_log: Arc::new(Mutex::new(Logger::new())),
      debug: *opts.debug(),
      seq,
    }
  }

  pub async fn run_server(&mut self) -> std::io::Result<()> {
    if self.debug {
      println!("Server running on {}:{}", self.ip, self.port);
    }
    let server_log = &mut self.server_log;
    loop {
      let log_copy = Arc::clone(server_log);
      let (stream, addr) = if self.seq {
        block_on(async { self.listener.accept().await.unwrap() })
      } else {
        self.listener.accept().await?
      };
      let debug = self.debug.clone();
      if debug {
        println!("New client: {}", addr);
      }
      let seq = self.seq.clone();
      if seq {
        block_on(async {
          Self::handle_client(&log_copy, stream, debug, seq).await;
        });
      } else {
        tokio::spawn(async move {
          Self::handle_client(&log_copy, stream, debug, seq).await;
        });
      }
    }
  }

  async fn verify_client_handshake(stream: &mut TcpStream, seq: bool) -> bool {
    let mut buf = [0; 1024];
    let size = if seq {
      block_on(async { stream.read(&mut buf).await.unwrap() })
    } else {
      stream.read(&mut buf).await.unwrap()
    };
    let request = String::from_utf8_lossy(&buf[..size]);
    let lines: std::vec::Vec<&str> = request.split('\n').collect();
    let first_line: vec::Vec<&str> = lines[0].split(' ').collect();
    let last_word = format!(r"{}", first_line[2]);
    if first_line.len() != 3
      || first_line[0] != "GET"
      || !first_line[1].starts_with('/')
      || last_word.trim() != r"HTTP/1.1"
    {
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
    if seq {
      block_on(async { stream.write(response.as_bytes()).await.unwrap() });
    } else {
      stream.write(response.as_bytes()).await.unwrap();
    }
    true
  }

  pub async fn read_message(
    server_log: &Arc<Mutex<Logger>>,
    buf: &mut Vec<u8>,
    stream: &mut TcpStream,
    debug: bool,
    seq: bool,
  ) -> bool {
    let result = if seq {
      block_on(async {
        match stream.read(buf).await {
          Ok(size) => {
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
          }
          Err(err) => {
            println!("{}", err);
            return false;
          }
        }
        true
      })
    } else {
      match stream.read(buf).await {
        Ok(size) => {
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
        }
        Err(err) => {
          println!("{}", err);
          return false;
        }
      }
      true
    };
    result
  }

  pub async fn write_message(
    server_log: &Arc<Mutex<Logger>>,
    buf: &mut Vec<u8>,
    stream: &mut TcpStream,
    seq: bool,
  ) -> bool {
    let result = if seq {
      block_on(async {
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
            block_on(async { stream.shutdown().await.unwrap() });
            false
          }
        }
      })
    } else {
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
    };
    result
  }

  pub async fn handle_client(
    server_log: &Arc<Mutex<Logger>>,
    mut stream: TcpStream,
    debug: bool,
    seq: bool,
  ) {
    let mut buf: Vec<u8> = vec![0; 1024];
    let handshake_success: bool = if seq {
      block_on(async { Self::verify_client_handshake(&mut stream, seq).await })
    } else {
      Self::verify_client_handshake(&mut stream, seq).await
    };
    if handshake_success {
      if seq {
        while block_on(async {
          Self::read_message(server_log, &mut buf, &mut stream, debug, seq).await
        }) {
          if !block_on(async { Self::write_message(server_log, &mut buf, &mut stream, seq).await })
          {
            break;
          }
        }
      } else {
        while Self::read_message(server_log, &mut buf, &mut stream, debug, seq).await {
          if !Self::write_message(server_log, &mut buf, &mut stream, seq).await {
            break;
          }
        }
      }
    } else {
      println!("Invalid client handshake");
    }
    if debug {
      println!("client all done");
    }
    let logger = server_log.lock().unwrap();
    logger.print_log();
  }
}
