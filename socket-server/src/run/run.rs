use crate::server::concurrent::ConcurrentServer;
use crate::utils::utils::Opts;
use tokio::spawn;

pub async fn run(opts: Opts) {
  let debug = *opts.debug();
  let mut my_server =
    ConcurrentServer::new(String::from("::1"), 8080, "1234567890".to_string(), opts).await;
  my_server.run_server().await.unwrap();
}