use crate::utils::logging::*;
use crate::utils::utils::{sec_websocket_key, Opts};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::vec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

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

pub fn pack_message_frame(payload: String) -> Vec<u8> {
  // FIN = 1 (only frame in message), RSV1-3 = 0, opcode = 0001 (text frame)
  let mut frame: Vec<u8> = vec![0b10000001]; 
  frame.reserve(1024);

  let mut second_byte: u8 = 0;
  let strlen = payload.len() as u64;
  let mut len_bytes = 0;
  //if from_client { // set mask bit
    //second_byte += 1 << 7;
  //}
  if strlen > 65535 { // 8 byte payload len
    second_byte += 127;
    len_bytes = 8;
  } else if strlen > 125 { // 2 byte payload len
    second_byte += 126;
    len_bytes = 2;
  } else {
    second_byte += (strlen) as u8;
  }
  frame.push(second_byte);

  if len_bytes == 8 {
    let bytes = u64::to_be_bytes(strlen);
    frame.extend_from_slice(&bytes);
  } else if len_bytes == 2 {
    let bytes = u16::to_be_bytes(strlen as u16);
    frame.extend_from_slice(&bytes);
  }

  frame.extend_from_slice(payload.as_bytes());
  frame
}

fn unpack_client_frame(buf: &mut Vec<u8>) -> Option<String> {
  println!("in the client frame");
  let first_byte = buf[0];
  let fin: bool = (first_byte & 128) >> 7 == 1;
  if !fin { // change
    return None;
  }
  let rsv: u8 = first_byte & 0b01110000;
  if rsv != 0 {
    return None;
  }
  let opcode: u8 = first_byte & 15;
  if opcode != 1 { // text frame, change this later
    println!("opcode {}", opcode);
    return None;
  }

  let second_byte = buf[1];
  let mask: bool = (second_byte & 128) >> 7 == 1;
  if !mask { // clients must mask stuff
    return None;
  }
  let second_byte_payload_len = second_byte & 127;
  let mut payload_len: usize = second_byte_payload_len as usize;
  let mut payload_len_bytes: usize = 0;
  if second_byte_payload_len == 127 {
    payload_len_bytes = 8;
    payload_len = u64::from_be_bytes(buf[2..10].try_into().unwrap()) as usize;
  } else if second_byte_payload_len == 126 {
    payload_len_bytes = 2;
    payload_len = u16::from_be_bytes(buf[2..4].try_into().unwrap()) as usize;
  }

  let mask_key_start = payload_len_bytes + 2;
  let mut masking_key: Vec<u8> = vec![0;4];
  masking_key.clone_from_slice(&buf[mask_key_start..mask_key_start+4]);
  
  let payload = &mut buf[mask_key_start+4..mask_key_start+4+payload_len];
  for i in 0..payload_len {
    payload[i] ^= masking_key[i % 4];
  }
  let s = (*String::from_utf8_lossy(payload)).to_string();

  Some(s)
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

  async fn verify_client_handshake(stream: &mut TcpStream) -> bool {
    let mut buf = [0; 1024];
    let size = stream.read(&mut buf).await.unwrap();
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
    let key = String::from(m.get("Sec-WebSocket-Key").unwrap().to_owned().trim());
    //println!("{}", key);
    let version = m.get("Sec-WebSocket-Version").unwrap().to_owned();
    let origin = m.get("Origin").unwrap().to_owned();

    if upgrade.trim() != "websocket" || connection.trim() != "Upgrade" || version.trim() != "13" {
      return false;
    }

    let my_key = sec_websocket_key(key);
    let response: String = format!(
      "HTTP/1.1 101 Switching Protocols\r\n\
      Upgrade: websocket\r\n\
      Connection: Upgrade\r\n\
      Sec-WebSocket-Accept: {}\r\n\r\n",
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
    match stream.read(buf).await {
      Ok(size) => {
        if size == 0 {
          if debug {
            println!("size is 0");
          }
          return false;
        }

        let msg = unpack_client_frame(buf);
        match msg {
          None => {
            return false;
          }
          Some(payload) => {
            let log_msg: String = format!("Server Read: {}", &payload);
            let m: Message = Message::new(log_msg.clone(), ErrorLevel::INFO);
            let mut logger = server_log.lock().unwrap();
            logger.log(m);
          }
        }

        
      }
      Err(err) => {
        println!("{}", err);
        return false;
      }
    }
    true
  }

  pub async fn write_message(
    server_log: &Arc<Mutex<Logger>>,
    message: &String,
    stream: &mut TcpStream,
  ) -> bool {
    let buf = pack_message_frame(message.clone());
    match stream.write(&buf).await {
      Ok(_) => {
        let msg: String = format!("Server Write: {}", message);
        let m: Message = Message::new(msg.clone(), ErrorLevel::INFO);
        let mut logger = server_log.lock().unwrap();
        logger.log(m);
        true
      }
      Err(_) => {
        //let peer = stream.peer_addr().expect("JFL");
        println!("An error occurred while writing");
        /*println!(
          "An error occurred while writing, terminating connection with {}",
          peer
        );*/
        stream.shutdown().await.unwrap();
        false
      }
    }
  }

  pub async fn handle_client(server_log: &Arc<Mutex<Logger>>, mut stream: TcpStream, debug: bool) {
    let mut buf: Vec<u8> = vec![0; 1024];
    let reply = String::from("YOYOYO");
    let handshake_success: bool = Self::verify_client_handshake(&mut stream).await;
    if handshake_success {
      Self::write_message(server_log, &reply, &mut stream).await;
      while Self::read_message(server_log, &mut buf, &mut stream, debug).await {
        if !Self::write_message(server_log, &reply, &mut stream).await {
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
