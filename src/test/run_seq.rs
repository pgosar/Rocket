use crate::test::testclient;
use crate::server::server;
use std::thread;

pub fn run() {
  let mut my_server = server::Server::new(String::from("::1"), 8080, "1234567890".to_string());
  let server_thread = thread::spawn(move || {
    my_server.run_server().expect("Error running server");
  });
  thread::sleep(std::time::Duration::from_secs(1));

  let mut my_client = testclient::TestClient::new(String::from("localhost:8080"));
  let mut my_other_client = testclient::TestClient::new(String::from("localhost:8080"));
  let client_thread = thread::spawn(move || {
    my_client
      .run_client(String::from("Hello World"), 2)
      .expect("Error running client");
  });
  let other_client_thread = thread::spawn(move || {
    my_other_client
      .run_client(String::from("Hello World"), 2)
      .expect("Error running client");
  });
  client_thread.join().expect("Error joining client thread");
  other_client_thread.join().expect("Error joining client thread");
  server_thread.join().expect("Error joining server thread");
}
