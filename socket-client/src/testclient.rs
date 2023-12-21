use crate::clientsocket::ClientSocket;
use rand::seq::index::sample;
use std::vec::Vec;
use tracing::info;

pub struct TestClient {
  socket: ClientSocket,
  id: u32,
}

impl TestClient {
  pub fn new(uri: String, id: u32) -> TestClient {
    let socket: ClientSocket = ClientSocket::new(uri);
    TestClient { id, socket }
  }

  pub async fn run_client(
    &mut self,
    msg: String,
    repeats: u32,
    num_clients: usize,
    out_degree: usize,
    sleep_mean: u32,
    sleep_padding: u64,
  ) -> std::io::Result<()> {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_entropy();
    self.socket.connect(self.id).await;
    tokio::time::sleep(std::time::Duration::from_millis(sleep_padding)).await;
    for _ in 0..repeats {
      let recipients: Vec<usize> = sample(&mut rng, num_clients, out_degree).into_vec();
      info!(
        "Client socket {} sending message to {:?}",
        self.id, recipients
      );
      self.socket.write_message(recipients, msg.clone()).await;
      tokio::time::sleep(std::time::Duration::from_millis(sleep_mean as u64)).await;
    }
    tokio::time::sleep(std::time::Duration::from_millis(sleep_padding)).await;
    self.socket.disconnect().await;
    Ok(())
  }
}
