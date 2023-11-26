use crate::server::concurrent::ConcurrentServer;
use crate::test::testclient;
use crate::utils::utils::Opts;
use pollster::block_on;
use tokio::spawn;

pub async fn run(opts: Opts) {
  let debug = *opts.debug();
  let repeats = *opts.repeats();
  let seq = *opts.mode() == "s";

  let mut my_server =
    ConcurrentServer::new(String::from("::1"), 8080, "1234567890".to_string(), opts).await;

  let server_thread = if seq {
    spawn(async move {
      block_on(my_server.run_server()).unwrap();
    })
  } else {
    spawn(async move {
      my_server.run_server().await.unwrap();
    })
  };

  let client_thread = if seq {
    spawn(async move {
      let mut my_client =
        testclient::TestClient::new(String::from("localhost:8080"), debug, seq).await;
      block_on(my_client.run_client(String::from("Hello World"), repeats, seq)).unwrap();
    })
  } else {
    spawn(async move {
      let mut my_client =
        testclient::TestClient::new(String::from("localhost:8080"), debug, seq).await;
      my_client
        .run_client(String::from("Hello World"), repeats, seq)
        .await
        .unwrap();
    })
  };

  let client_thread_2 = if seq {
    spawn(async move {
      let mut my_client =
        testclient::TestClient::new(String::from("localhost:8080"), debug, seq).await;
      block_on(my_client.run_client(String::from("Other Client"), repeats, seq)).unwrap();
    })
  } else {
    spawn(async move {
      let mut my_client =
        testclient::TestClient::new(String::from("localhost:8080"), debug, seq).await;
      my_client
        .run_client(String::from("Other Client"), repeats, seq)
        .await
        .unwrap();
    })
  };
  if seq {
    block_on(server_thread).unwrap();
    block_on(client_thread).unwrap();
    block_on(client_thread_2).unwrap();
  } else {
    server_thread.await.unwrap();
    client_thread.await.unwrap();
    client_thread_2.await.unwrap();
  }
}
