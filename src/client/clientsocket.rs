use base64::{engine::general_purpose, Engine};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;
use std::collections::HashMap;
use std::vec::Vec;
use crate::utils::utils::*;



pub struct ClientSocket {
  server_uri: String,
  server_port: u16,
  server_path: String,
  stream: Option<TcpStream>,
  reader_thread: Option<thread::JoinHandle<()>>,
  sender: Option<mpsc::Sender<()>>,
}

fn generate_key() -> String {
  // Random 16 byte value base-64 encoded
  let bytes: String = (0..16).map(|_| rand::random::<u8>() as char).collect();
  general_purpose::STANDARD.encode(&bytes)
}

impl ClientSocket {
  pub fn new(uri: String) -> ClientSocket {
    let split_uri: Vec<&str> = uri.split(':').collect();
    let port_path = String::from(split_uri[1]);
    let port_path_vec: Vec<&str> = port_path.split('/').collect();
    let mut path = String::from("");
    if port_path_vec.len() > 1 {
      path = String::from(port_path_vec[1]);
    }
    let server_uri = String::from(split_uri[0]);
    let server_port = port_path_vec[0].parse::<u16>().unwrap();
    ClientSocket {
      server_uri,
      server_port,
      server_path: path,
      stream: None,
      reader_thread: None,
      sender: None,
    }
  }

  fn handshake_http(&mut self) -> bool {
    //dGhlIHNhbXBsZSBub25jZQ==
    let mut stream = self.stream.as_ref().expect("Stream not instantiated")
                 .try_clone().expect("clone failed");
    let mut buf = vec![0; 1024];
    let my_addr: std::net::SocketAddr = stream.local_addr().unwrap();
    let my_key: String = generate_key();
    let handshake = format!(
      "GET {} HTTP/1.1\n\
      Host: {}:{}\n\
      Upgrade: websocket\n\
      Connection: Upgrade\n\
      Sec-WebSocket-Key: {}\n\
      Origin: {}:{}\n\
      Sec-WebSocket-Version: 13",
      self.server_path,
      self.server_uri,
      self.server_port,
      &my_key,
      my_addr.ip().to_string(),
      my_addr.port().to_string(),
    );
    match stream.write(handshake.as_bytes()) {
      Ok(_) => {}
      Err(e) => {
        println!("Failed to send handshake: {}", e);
        return false;
      }
    }
    stream.write(handshake.as_bytes()).expect("write failed");
    match stream.read(&mut buf) {
      Ok(size) => {
        let request = String::from_utf8_lossy(&buf[..size]);
        println!("{}", request);
        let lines: Vec<&str> = request.split('\n').collect();
        if lines[0] != "HTTP/1.1 101 Switching Protocol" {
          return false;
        }
        let mut m: HashMap<String, Option<String>> = HashMap::from([
          (String::from("Upgrade"), None),
          (String::from("Connection"), None),
          (String::from("Sec-WebSocket-Accept"), None)
        ]);
        for line in lines[1..].iter() {
          let split_line: Vec<&str> = (*line).split(": ").collect();
          if split_line.len() != 2 {
            return false;
          }
          let mut old = m.get(split_line[0]);
          if old.is_none() || old.take().is_some() { // each correct key should exist once and only once
            return false;
          }
          m.insert(String::from(split_line[0]), Some(String::from(split_line[1])));
        }
        let upgrade = m.get("Upgrade").to_owned().unwrap().to_owned().unwrap();
        let connection = m.get("Connection").to_owned().unwrap().to_owned().unwrap();
        let swk = m.get("Sec-WebSocket-Accept").to_owned().unwrap().to_owned().unwrap();
        if upgrade != "websocket" || connection != "Upgrade" 
          || swk != sec_websocket_key(my_key) {
          return false;
        }


        // HTTP/1.1 101 Switching Protocol
        // expected = Upgrade, Connection, Sec-WebSocket-Accept
      }
      Err(e) => {
        println!("Failed to receive data: {}", e);
        return false;
      }
    }
    true
  }

  fn reader_loop(mut stream: TcpStream, receiver: mpsc::Receiver<()>) {
    let mut buf = vec![0; 1024];
    stream.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
    loop {
      match receiver.try_recv() {
        Ok(_) | Err(TryRecvError::Disconnected) => {
          break;
        }
        Err(TryRecvError::Empty) => {
          match stream.read(&mut buf) {
            Ok(_) => {
              println!("Client Received: {}", from_utf8(&buf).unwrap());
            }
            Err(e) => {
              //println!("Failed to receive data: {}", e);
            }
          }
        }
      }
    }
  }

  pub fn write_message(&self, msg: String) {
    let mut stream = self.stream.as_ref().expect("Stream not instantiated")
                 .try_clone().expect("clone failed");
    let byte_msg = msg.as_bytes();
    stream.write(byte_msg).unwrap();
    println!("Client Sent: {}", msg);
  }

  pub fn connect(&mut self) {
    let address: String = format!("{}:{}", self.server_uri, self.server_port);
    println!("{}", address);
    match TcpStream::connect(address) {
      Ok(stream) => {
        println!(
          "Successfully connected to server in port {}",
          self.server_port
        );
        self.stream = Some(stream.try_clone().unwrap());
        if self.handshake_http() {
          let (tx, rx) = mpsc::channel();
          self.sender = Some(tx);
          self.reader_thread = Some(thread::spawn(move || {
            let _ = Self::reader_loop(stream, rx);
          }));
        }
      }
      Err(e) => {
        println!("Failed to receive data: {}", e);
      }
    }
  }

  pub fn disconnect(&mut self) {
    if let Some(tx) = self.sender.take() {
      tx.send(()).unwrap();
      if let Some(jh) = self.reader_thread.take() {
          jh.join().unwrap();
      }
    } 
  }
}
