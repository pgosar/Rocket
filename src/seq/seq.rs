use crate::seq::client;
use crate::seq::server;
use std::thread;

pub fn run() {
    let server_thread = thread::spawn(|| {
        server::run_server().expect("Error running server");
    });
    thread::sleep(std::time::Duration::from_secs(1));
    let my_client = client::Client::new(String::from("::1"), 8080); 
    let client_thread = thread::spawn(move || {
        my_client.run_client(String::from("Hello World"), 2).expect("Error running client");
    });
    client_thread.join().expect("Error joining client thread");
    thread::sleep(std::time::Duration::from_secs(1));
    let my_client = client::Client::new(String::from("::1"), 8080); 
    let client_thread = thread::spawn(move || {
        my_client.run_client(String::from("Hello World"), 2).expect("Error running client");
    });
    client_thread.join().expect("Error joining client thread");
    server_thread.join().expect("Error joining server thread");

}
