use crate::client::clientsocket::ClientSocket;
use rand::seq::index::sample;
use std::vec::Vec;

pub struct TestClient {
  socket: ClientSocket,
  id: u32,
}

impl TestClient {
  pub async fn new(uri: String, id: u32, debug: bool) -> TestClient {
    let socket: ClientSocket = ClientSocket::new(uri, debug).await;
    TestClient { id, socket }
  }

  pub async fn run_client(
    &mut self,
    msg: String,
    repeats: u32,
    num_clients: usize,
    out_degree: usize,
  ) -> std::io::Result<()> {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_entropy();
    self.socket.connect(self.id).await;
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    for _ in 0..repeats {
      let recipients: Vec<usize> = sample(&mut rng, num_clients, out_degree).into_vec();
      println!("client socket {} sending to {:?}", self.id, recipients);
      self.socket.write_message(recipients, msg.clone()).await;
      tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    self.socket.disconnect().await;
    Ok(())
  }
}
