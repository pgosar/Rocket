use crate::client::clientsocket::ClientSocket;

pub struct TestClient {
  socket: ClientSocket,
}

impl TestClient {
  pub async fn new(uri: String, debug: bool) -> TestClient {
    let socket: ClientSocket = ClientSocket::new(uri, debug).await;
    TestClient { socket }
  }

  pub async fn run_client(&mut self, msg: String, repeats: i32) -> std::io::Result<()> {
    self.socket.connect().await;
    std::thread::sleep(std::time::Duration::from_secs(2));
    for _ in 0..repeats {
      self.socket.write_message(msg.clone()).await;
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
    self.socket.disconnect().await;
    Ok(())
  }
}
