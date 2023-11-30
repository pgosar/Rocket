use crate::utils::utils::*;
use base64::{engine::general_purpose, Engine};
use std::collections::HashMap;
use std::vec::Vec;
use tokio::task::JoinHandle;

use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct ClientSocket {
  server_uri: String,
  server_port: u16,
  server_path: String,
  write_stream: Option<Arc<Mutex<OwnedWriteHalf>>>,
  reader_thread: Option<JoinHandle<()>>,
  mask_key: Vec<u8>,
  connected: bool,
  debug: bool,
}

fn generate_key() -> String {
  // Random 16 byte value base-64 encoded
  let bytes: String = (0..16).map(|_| rand::random::<u8>() as char).collect();
  general_purpose::STANDARD.encode(&bytes)
}

fn pack_message_frame(payload: String, masking_key: &Vec<u8>) -> Vec<u8> {
  // FIN = 1 (only frame in message), RSV1-3 = 0, opcode = 0001 (text frame)
  let mut frame: Vec<u8> = vec![0b10000001];
  frame.reserve(1024);

  let mut second_byte: u8 = 128; // set mask bit
  let strlen = payload.len() as u64;
  let mut len_bytes = 0;
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
  frame.extend_from_slice(&masking_key);
  let payload_start = frame.len();
  frame.extend_from_slice(payload.as_bytes());
  for i in 0..(strlen as usize) {
    frame[i + payload_start] ^= masking_key[i % 4];
  }
  frame
}

fn unpack_server_frame(buf: &mut Vec<u8>) -> (Option<u8>, Option<String>) {
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
  if mask {
    // servers must not mask stuff
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

  let payload_start = payload_len_bytes + 2;

  let payload = &mut buf[payload_start..payload_start + payload_len];
  let s = (*String::from_utf8_lossy(payload)).to_string();

  (Some(opcode), Some(s))
}

impl ClientSocket {
  pub async fn new(uri: String, debug: bool) -> ClientSocket {
    let split_uri: std::vec::Vec<&str> = uri.split(':').collect();
    let port_path = String::from(split_uri[1]);
    let port_path_vec: Vec<&str> = port_path.split('/').collect();
    let mut path = String::from("/");
    if port_path_vec.len() > 1 {
      path = String::from("/") + port_path_vec[1];
    }
    let server_uri = String::from(split_uri[0]);
    let server_port = port_path_vec[0].parse::<u16>().unwrap();
    if debug {
      println!("{} {} {}", server_uri, server_port, path);
    }
    ClientSocket {
      server_uri: server_uri.clone(),
      server_port,
      server_path: path,
      write_stream: None,
      reader_thread: None,
      mask_key: vec![0; 4],
      debug,
      connected: false,
    }
  }

  async fn handshake_http(
    &mut self,
    read_half: &mut OwnedReadHalf,
    write_half: &mut OwnedWriteHalf,
  ) -> bool {
    //dGhlIHNhbXBsZSBub25jZQ==
    let mut buf = vec![0; 1024];
    let my_addr: std::net::SocketAddr = read_half.local_addr().unwrap();
    let my_key: String = generate_key();
    let handshake = format!(
      "GET {} HTTP/1.1\r\n\
      Host: {}:{}]\r\n\
      Upgrade: websocket\r\n\
      Connection: Upgrade\r\n\
      Sec-WebSocket-Key: {}\r\n\
      Origin: {}:{}\r\n\
      Sec-WebSocket-Version: 13\r\n\r\n",
      self.server_path,
      self.server_uri,
      self.server_port,
      &my_key,
      my_addr.ip().to_string(),
      my_addr.port().to_string(),
    );
    match write_half.write(handshake.as_bytes()).await {
      Ok(_) => {}
      Err(e) => {
        println!("Failed to send handshake: {}", e);
        return false;
      }
    }
    /*stream
    .write(handshake.as_bytes())
    .await
    .expect("write failed");*/
    match read_half.read(&mut buf).await {
      Ok(size) => {
        let request = String::from_utf8_lossy(&buf[..size]);
        let lines: Vec<&str> = request.trim().split("\r\n").collect();
        if lines[0].trim() != "HTTP/1.1 101 Switching Protocols" {
          return false;
        }
        let mut m: HashMap<String, Option<String>> = HashMap::from([
          (String::from("Upgrade"), None),
          (String::from("Connection"), None),
          (String::from("Sec-WebSocket-Accept"), None),
        ]);
        for line in lines[1..].iter() {
          let split_line: Vec<&str> = (*line).trim().split(": ").collect();
          if split_line.len() != 2 {
            return false;
          }
          let old = m.get(split_line[0]);
          if old.is_none()
          /*|| old.take().is_some()*/
          {
            // each correct key should exist once and only once
            //println!("invalid key {} {} {}", split_line[0], old.is_none(), val);
            return false;
          }
          m.insert(
            String::from(split_line[0]),
            Some(String::from(split_line[1])),
          );
        }
        let upgrade = m.get("Upgrade").to_owned().unwrap().to_owned().unwrap();
        let connection = m.get("Connection").to_owned().unwrap().to_owned().unwrap();
        let swk = m
          .get("Sec-WebSocket-Accept")
          .to_owned()
          .unwrap()
          .to_owned()
          .unwrap();
        let expected_key = sec_websocket_key(my_key);
        if upgrade != "websocket" || connection != "Upgrade" || swk != expected_key {
          return false;
        }
      }
      Err(e) => {
        if self.debug {
          println!("Failed to receive data: {}", e);
        }
        return false;
      }
    }
    if self.debug {
      println!("Handshake succeeded");
    }
    true
  }

  async fn reader_loop(
    read_stream: &mut OwnedReadHalf,
    write_stream: &Arc<Mutex<OwnedWriteHalf>>,
    debug: bool,
  ) {
    println!("beginning of reader loop");
    let mut buf = vec![0; 1024];
    loop {
      match read_stream.read(&mut buf).await {
        Ok(size) => {
          if size == 0 {
            if debug {
              println!("size is 0");
            }
            break;
          }
          let (opcode, payload) = unpack_server_frame(&mut buf);
          match opcode {
            None => {
              break;
            }
            Some(opcode_val) => {
              if opcode_val == 0x8 {
                // send a closing frame too if you have not already sent one
                if debug {
                  println!("Client received closing frame");
                }
                break;
              } else if opcode_val == 0x9 {
                // ping, send pong
                // send control frame of 0xA
                Self::send_control_frame(write_stream, 0xA, debug).await;
                if debug {
                  println!("Client received ping");
                }
              } else if opcode_val == 0x1 {
                match payload {
                  None => {}
                  Some(msg) => {
                    if debug {
                      println!("Client Received: {}", &msg);
                    }
                  }
                }
              } else {
                // handle pings and pongs
                break;
              }
            }
          }
        }
        Err(e) => {
          if debug {
            println!("Failed to receive data: {}", e);
          }
          break;
        }
      }
    }
    println!("end of reader loop");
  }

  pub async fn write_message(&mut self, recipients: Vec<usize>, msg: String) {
    if !self.connected {
      panic!("Client not connected");
    }
    let combined_msg = recipients.iter().map(|x| x.to_string()).collect::<String>() + "," + &msg;
    let byte_msg = pack_message_frame(combined_msg.clone(), &self.mask_key);
    let mut stream = self.write_stream.as_mut().unwrap().lock().await;
    match stream.write(&byte_msg).await {
      Ok(_) => {
        if self.debug {
          println!("Client Sent: {}", msg);
        }
      }
      Err(err) => {
        if self.debug {
          println!("Error writing from client: {}", err);
        }
      }
    }
  }

  pub async fn connect(&mut self, id: u32) {
    let address: String = format!("{}:{}", self.server_uri, self.server_port);
    if self.debug {
      println!("Connecting to {}", address);
    }
    match TcpStream::connect(address).await {
      Ok(stream) => {
        let (mut read_half, mut write_half) = stream.into_split();
        self.connected = self.handshake_http(&mut read_half, &mut write_half).await;
        if self.connected {
          self.write_stream = Some(Arc::new(Mutex::new(write_half)));
          if self.debug {
            println!(
              "Successfully connected to server in port {}",
              self.server_port
            );
          }
          self.write_message(Vec::new(), id.to_string()).await;
          //let (tx, rx) = channel(8);
          //self.sender = Some(tx);
          let debug = self.debug;
          //let mut stream_clone = Arc::clone(self.write_stream.as_mut().unwrap());
          //let mut recv_clone = Arc::new(Mutex::new(rx));
          let stream_clone = Arc::clone(&self.write_stream.as_ref().unwrap());
          self.reader_thread = Some(tokio::spawn(async move {
            Self::reader_loop(&mut read_half, &stream_clone, debug).await
          }));
          for i in 0..4 {
            self.mask_key[i] = rand::random::<u8>()
          }
        } else {
          if self.debug {
            println!("invalid server handshake");
          }
        }
      }
      Err(e) => {
        if self.debug {
          println!("Failed to connect stream: {}", e);
        }
      }
    }
  }

  async fn send_control_frame(write_stream: &Arc<Mutex<OwnedWriteHalf>>, opcode: u8, debug: bool) {
    let byte_msg: Vec<u8> = vec![0b10000000 + opcode];
    let mut stream = write_stream.lock().await;
    match stream.write(&byte_msg).await {
      Ok(_) => {
        if debug {
          println!("Client Sent opcode {} ", opcode);
        }
      }
      Err(_) => {
        if debug {
          println!("Failed to send client control frame of code {}", opcode);
        }
      }
    }
  }

  pub async fn disconnect(&mut self) {
    // if let Some(tx) = self.sender.take() {
    //tx.send(()).await.unwrap();
    Self::send_control_frame(self.write_stream.as_mut().unwrap(), 8, self.debug).await;
    if let Some(jh) = self.reader_thread.take() {
      jh.await.unwrap();
    }
    self
      .write_stream
      .as_mut()
      .expect("Stream not instantiated")
      .lock()
      .await
      .shutdown()
      .await
      .unwrap();

    println!("disconnecting client");
  }
}
