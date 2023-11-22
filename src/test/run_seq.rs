use crate::server::concurrent::ConcurrentServer;
use crate::test::testclient;
use tokio::spawn;

pub async fn run() {
  
  let mut my_server =
    ConcurrentServer::new(String::from("::1"), 8080, "1234567890".to_string()).await;
  
  let server_thread = spawn(async move {
    my_server.run_server().await.unwrap();
  });

  let client_thread = spawn(async move {
    let mut my_client = testclient::TestClient::new(String::from("localhost:8080")).await;
    my_client
      .run_client(String::from("Hello World"), 2)
      .await
      .unwrap();
  });

  server_thread.await.unwrap();
  client_thread.await.unwrap();

}
