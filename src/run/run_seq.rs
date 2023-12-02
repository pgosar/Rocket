use crate::run::testclient;
use crate::server::concurrent::ConcurrentServer;
use crate::utils::utils::Opts;
use tokio::spawn;

pub async fn run(opts: Opts) {
  let debug = *opts.debug();
  let repeats = *opts.repeats();

  let mut my_server =
    ConcurrentServer::new(String::from("::1"), 8080, "1234567890".to_string(), opts).await;

  let server_thread = spawn(async move {
    my_server.run_server().await.unwrap();
  });

  let client_thread = spawn(async move {
    let mut my_client = testclient::TestClient::new(String::from("localhost:8080"), debug).await;
    my_client
      .run_client(String::from("Hello World"), repeats)
      .await
      .unwrap();
  });

  let client_thread_2 = spawn(async move {
    let mut my_client = testclient::TestClient::new(String::from("localhost:8080"), debug).await;
    my_client
      .run_client(String::from("Other Client"), repeats)
      .await
      .unwrap();
  });

  server_thread.await.unwrap();
  client_thread.await.unwrap();
  client_thread_2.await.unwrap();
}
