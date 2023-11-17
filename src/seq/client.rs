use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub struct Client {
  ip: String,
  port: u16,
}

impl Client {
  pub fn new(ip: String, port: u16) -> Client {
    Client { ip: ip, port: port }
  }
  pub fn run_client(&self, msg: String, repeats: i32) -> std::io::Result<()> {
    let address: String = format!("[{}]:{}", self.ip, self.port);
    match TcpStream::connect(address) {
      Ok(mut stream) => {
        println!("Successfully connected to server in port {}", self.port);
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
