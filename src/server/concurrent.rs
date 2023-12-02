use crate::server::connectedclient::ConnectedClient;
use crate::utils::logging::*;
use crate::utils::utils::{sec_websocket_key, Opts};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::vec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};

type ClientMap = Arc<RwLock<HashMap<u32, Mutex<ConnectedClient>>>>;

async fn create_listener(ip: String, port: u16) -> TcpListener {
  let address: SocketAddr = format!("[{}]:{}", ip, port).parse().unwrap();
  let listener: TcpListener = TcpListener::bind(address).await.unwrap();
  listener
}

fn pack_message_frame(payload: String) -> Vec<u8> {
  // FIN = 1 (only frame in message), RSV1-3 = 0, opcode = 0001 (text frame)
  let mut frame: Vec<u8> = vec![0b10000001];
  frame.reserve(1024);

  let mut second_byte: u8 = 0;
  let strlen = payload.len() as u64;
  let mut len_bytes = 0;
  //if from_client { // set mask bit
  //second_byte += 1 << 7;
  //}
  if strlen > 65535 {
    // 8 byte payload len
    second_byte += 127;
    len_bytes = 8;
  } else if strlen > 125 {
    // 2 byte payload len
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

fn unpack_client_frame(buf: &mut Vec<u8>) -> (Option<u8>, Option<String>) {
  let first_byte = buf[0];
  let fin: bool = (first_byte & 128) >> 7 == 1;
  if !fin {
    // change
    return (None, None);
  }
  let rsv: u8 = first_byte & 0b01110000;
  if rsv != 0 {
    return (None, None);
  }
  let opcode: u8 = first_byte & 15;
  if opcode != 1 {
    // text frame, change this later
    return (Some(opcode), None);
  }

  let second_byte = buf[1];
  let mask: bool = (second_byte & 128) >> 7 == 1;
  if !mask {
    // clients must mask stuff
    return (None, None);
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
  let mut masking_key: Vec<u8> = vec![0; 4];
  masking_key.clone_from_slice(&buf[mask_key_start..mask_key_start + 4]);

  let payload = &mut buf[mask_key_start + 4..mask_key_start + 4 + payload_len];
  for i in 0..payload_len {
    payload[i] ^= masking_key[i % 4];
  }
  let s = (*String::from_utf8_lossy(payload)).to_string();

  (Some(opcode), Some(s))
}

#[derive(Debug)]
pub struct ConcurrentServer {
  ip: String,
  port: u16,
  key: String,
  listener: TcpListener,
  server_log: Arc<Mutex<Logger>>,
  opts: Opts,
  clients: ClientMap,
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
      clients: ClientMap::new(RwLock::new(HashMap::new())),
    }
  }

  pub async fn run_server(&mut self) -> std::io::Result<()> {
    if *self.opts.debug() {
      println!("Server running on {}:{}", self.ip, self.port);
    }
    let server_log = &mut self.server_log;
    loop {
      let log_copy = Arc::clone(server_log);
      let clients_copy = Arc::clone(&self.clients);

      let (stream, addr) = self.listener.accept().await?;
      if *self.opts.debug() {
        println!("New client: {}", addr);
      }
      let debug = (self.opts.debug()).clone();
      tokio::spawn(async move {
        Self::handle_client(&log_copy, stream, clients_copy, debug).await;
        // Self::send_heartbeat(Arc::new(Mutex::new(stream)), debug).await;
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
    stream: &mut OwnedReadHalf,
    debug: bool,
  ) -> (Option<u8>, Option<String>) {
    match stream.read(buf).await {
      Ok(size) => {
        if size == 0 {
          if debug {
            println!("server: size is 0");
          }
          return (None, None);
        }

        let (opcode, payload) = unpack_client_frame(buf);
        match opcode {
          None => {
            return (None, None);
          }
          Some(opcode_val) => {
            if opcode_val == 0x1 {
              match payload {
                None => {
                  return (opcode, payload);
                }
                Some(msg) => {
                  let log_msg: String = format!("Server Read: {}", &msg);
                  let m: Message = Message::new(log_msg.clone(), ErrorLevel::INFO);
                  let mut logger = server_log.lock().await;
                  logger.log(m);
                  return (opcode, Some(msg));
                }
              }
            } else {
              return (opcode, payload);
            }
          }
        }
      }
      Err(err) => {
        println!("{}", err);
        return (None, None);
      }
    }
  }

  pub async fn write_message(
    client_ids: Vec<u32>,
    all_clients: &ClientMap,
    server_log: &Arc<Mutex<Logger>>,
    message: &String,
  ) -> bool {
    let buf = pack_message_frame(message.clone());
    //println!("not running??");
    println!("Sending to {:?}", client_ids);
    for client in client_ids {
      //println!("running");
      let client_map = all_clients.read().await;
      let client_object = client_map.get(&client).unwrap().lock().await;
      let mut client_stream = client_object.stream().lock().await;
      //println!("buf: {:?}", buf);
      match (*client_stream).write(&buf).await {
        Ok(_) => {
          let msg: String = format!("Server Write: {}", message);
          let m: Message = Message::new(msg.clone(), ErrorLevel::INFO);
          let mut logger = server_log.lock().await;
          logger.log(m);
          // return true;
        }
        Err(_) => {
          println!("An error occurred while writing, terminating connection");
          client_stream.shutdown().await.unwrap();
          return false;
        }
      }
    }
    true
  }

  /*async fn send_heartbeat(stream: &mut TcpStream, debug: bool) {
    // let mut unwrap_stream = Arc::try_unwrap(stream).unwrap().into_inner().unwrap();
    loop {
      Self::send_control_frame(stream, 0x9, debug).await;
      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
      if debug {
        println!("server sent heartbeat");
      }
    }
  }*/

  async fn send_control_frame(stream: &mut OwnedWriteHalf, opcode: u8, debug: bool) {
    let byte_msg: Vec<u8> = vec![0b10000000 + opcode];
    if opcode == 0x9 {
      // Self::send_heartbeat(stream, debug);
    }
    match stream.write(&byte_msg).await {
      Ok(_) => {
        if debug {
          println!("Server sent opcode {} ", opcode);
        }
      }
      Err(_) => {
        if debug {
          println!("Failed to send server control frame of code {}", opcode);
        }
      }
    }
  }

  pub async fn handle_client(
    server_log: &Arc<Mutex<Logger>>,
    mut stream: TcpStream,
    clients: ClientMap,
    debug: bool,
  ) {
    let mut buf: Vec<u8> = vec![0; 1024];
    let handshake_success: bool = Self::verify_client_handshake(&mut stream).await;
    if handshake_success {
      // Self::send_heartbeat(Arc::new(Mutex::new(stream)), debug).await;
      let (mut read_half, mut write_half) = stream.into_split();
      let (_, first_data) = Self::read_message(server_log, &mut buf, &mut read_half, debug).await;
      println!("first data: {}", first_data.clone().unwrap());
      let id = first_data.unwrap().parse::<u32>().expect("Invalid id");
      let mut client_map = clients.write().await;

      let write_half_arc = Arc::new(Mutex::new(write_half));
      client_map.insert(
        id,
        Mutex::new(ConnectedClient::new(id, Arc::clone(&write_half_arc))),
      );
      std::mem::drop(client_map);

      loop {
        let (opcode, data) = Self::read_message(server_log, &mut buf, &mut read_half, debug).await;
        if opcode.is_none() {
          break;
        }
        let opcode_val = opcode.unwrap();
        if opcode_val == 0x8 {
          if debug {
            println!("Server received opcode 8");
          }
          let mut wh = write_half_arc.lock().await;
          Self::send_control_frame(&mut wh, opcode_val, debug).await;
          break;
        } else if opcode_val == 0x9 {
          let mut wh = write_half_arc.lock().await;
          Self::send_control_frame(&mut wh, 0xA, debug).await;
        } else if opcode_val == 0x1 {
          let unwrapped_data = data.unwrap();
          let split_data: Vec<&str> = unwrapped_data.split(',').collect();
          let text_message = String::from(split_data[split_data.len() - 1]);
          let ids: Vec<u32> = split_data[0..split_data.len() - 1]
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
          if !Self::write_message(ids, &clients, server_log, &text_message).await {
            break;
          }
        } else {
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
    let logger = server_log.lock().await;
    logger.print_log().unwrap();
  }
}
