use crate::seq::clientsocket;
use clientsocket::ClientSocket;

pub struct TestClient {
  socket: ClientSocket,
} 

impl TestClient {
  pub fn new(uri: String) -> TestClient {
    let socket: ClientSocket = ClientSocket::new(uri);
    TestClient {
      socket
    }
  }

  pub fn run_client(&mut self, msg: String, repeats: i32) -> std::io::Result<()> {
    self.socket.connect();
    for _ in 0..repeats {
      self.socket.write_message(msg.clone());
      std::thread::sleep(std::time::Duration::from_secs(2));
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
    self.socket.disconnect();
    Ok(())
  }
}
