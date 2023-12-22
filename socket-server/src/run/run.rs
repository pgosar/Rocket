use crate::server::concurrent::ConcurrentServer;

pub async fn run() {
  let mut my_server =
    ConcurrentServer::new(String::from("::1"), 8080, "1234567890".to_string()).await;
  my_server.run_server().await.unwrap();
}
