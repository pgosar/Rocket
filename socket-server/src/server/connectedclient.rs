use getset::Getters;
use std::sync::Arc;
use tokio::net::tcp::OwnedWriteHalf;

use tokio::sync::Mutex;

#[derive(Debug, Getters)]
pub struct ConnectedClient {
  id: u32,
  heartbeat_status: bool,
  connected_status: bool,
  last_ping_time: u32,
  #[getset(get = "pub")]
  stream: Arc<Mutex<OwnedWriteHalf>>,
}

impl ConnectedClient {
  pub fn new(id: u32, stream: Arc<Mutex<OwnedWriteHalf>>) -> ConnectedClient {
    let client = ConnectedClient {
      id,
      heartbeat_status: false,
      connected_status: true,
      last_ping_time: 0,
      stream: Arc::clone(&stream),
    };
    let client_arc = Arc::new(Mutex::new(client));
    //let cloned_client = Arc::clone(&client_arc);
    /*tokio::spawn(async move {
      let mut client = cloned_client.lock().await;
      client.acknowledge_heartbeat().await;
    });*/

    Arc::try_unwrap(client_arc).unwrap().into_inner()
  }

  pub fn send_message(&self) {}

  /*pub async fn acknowledge_heartbeat(&mut self) {
    loop {
      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

      let mut stream = self.stream.lock().await;
      match stream.read(&mut [0; 128]).await {
        Ok(_) => {
          self.heartbeat_status = true;
        }
        Err(e) => {
          eprintln!("Error reading from stream: {}", e);
          self.connected_status = false;
          break;
        }
      }
    }
  }*/
}
