use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpStream;


pub struct ConnectedClient {
  id: u32,
  heartbeat_status: bool,
  connected_status: bool,
  stream: Arc<Mutex<TcpStream>>,
}

impl ConnectedClient {
  pub fn new(id: u32, stream: Arc<Mutex<TcpStream>>) -> ConnectedClient {
    ConnectedClient {
      id, 
      heartbeat_status: false, 
      connected_status: true, 
      stream
    }
  }

  pub fn send_message() {
    
  }

  pub fn acknowledge_heartbeat() {}
}