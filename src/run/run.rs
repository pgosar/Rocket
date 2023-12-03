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
  let sleep_mean: u32 = *opts.sleep_time_mean();
  let mut my_server =
    ConcurrentServer::new(String::from("::1"), 8080, "1234567890".to_string(), opts).await;
  let server_thread = spawn(async move {
    my_server.run_server().await.unwrap();
  });
  let mut join_handles: Vec<JoinHandle<()>> = Vec::new();
  let start = std::time::Instant::now();
  let total_subtracted = std::time::Duration::from_millis((4000 + sleep_mean * repeats).into());
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
        )
        .await
        .unwrap();

    });
    join_handles.push(thread);
  }

  for jh in join_handles.into_iter() {
    jh.await.expect("Client thread failed");
  }
  let end = std::time::Instant::now();
  // end to end runtime - client construction time - client sleep times
  let total_time = end
    .duration_since(start)
    .checked_sub(total_subtracted)
    .unwrap();
  println!("Total time: {:?}", total_time);
  server_thread.await.expect("Server thread failed");
}
