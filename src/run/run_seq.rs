use crate::run::testclient;
use crate::server::concurrent::ConcurrentServer;
use crate::utils::utils::Opts;
use std::vec::Vec;
use tokio::spawn;
use tokio::task::JoinHandle;

pub async fn run(opts: Opts) {
  let debug = *opts.debug();
  let repeats = *opts.repeats();
  let num_clients = *opts.num_clients();
  let out_degree = *opts.out_degree() as usize;
  let sleep_mean: f32 = *opts.sleep_time_mean();
  let sleep_std: f32 = *opts.sleep_time_std();
  let mut my_server =
    ConcurrentServer::new(String::from("::1"), 8080, "1234567890".to_string(), opts).await;
  let server_thread = spawn(async move {
    my_server.run_server().await.unwrap();
  });
  let mut join_handles: Vec<JoinHandle<()>> = Vec::new();
  for i in 0..num_clients as u32 {
    let thread = spawn(async move {
      let mut my_client =
        testclient::TestClient::new(String::from("localhost:8080"), i, debug).await;
      my_client
        .run_client(
          String::from("Hello World"),
          repeats,
          num_clients,
          out_degree,
          sleep_mean,
          sleep_std,
        )
        .await
        .unwrap();
    });
    join_handles.push(thread);
  }
  for jh in join_handles.into_iter() {
    jh.await.expect("Client thread failed");
  }
  server_thread.await.expect("Server thread failed");
}
