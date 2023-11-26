use crate::client::clientsocket::ClientSocket;
use pollster::block_on;

pub struct TestClient {
  socket: ClientSocket,
}

impl TestClient {
  pub async fn new(uri: String, debug: bool, seq: bool) -> TestClient {
    let socket: ClientSocket = if seq {
      block_on(ClientSocket::new(uri, debug, seq))
    } else {
      ClientSocket::new(uri, debug, seq).await
    };
    TestClient { socket }
  }

  pub async fn run_client(&mut self, msg: String, repeats: i32, seq: bool) -> std::io::Result<()> {
    if seq {
      block_on(async {
        self.socket.connect().await;
        for _ in 0..repeats {
          block_on(self.socket.write_message(msg.clone(), seq));
          std::thread::sleep(std::time::Duration::from_secs(2));
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
        self.socket.disconnect().await;
      });
    } else {
      self.socket.connect().await;
      for _ in 0..repeats {
        self.socket.write_message(msg.clone(), seq).await;
        std::thread::sleep(std::time::Duration::from_secs(2));
      }
      std::thread::sleep(std::time::Duration::from_secs(2));
      self.socket.disconnect().await;
    }
    Ok(())
  }
}
