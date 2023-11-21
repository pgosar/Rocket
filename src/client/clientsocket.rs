use base64::{engine::general_purpose, Engine};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;


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
    let split_uri: std::vec::Vec<&str> = uri.split(':').collect();
    let port_path = String::from(split_uri[1]);
    let port_path_vec: std::vec::Vec<&str> = port_path.split('/').collect();
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
    stream.write(handshake.as_bytes()).expect("write failed");
    match stream.read(&mut buf) {
      Ok(_) => {
        println!("Client Received: {}", from_utf8(&buf).unwrap());
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
