use crate::seq::client;
use crate::seq::server;
use std::thread;

pub fn run() {
    let server_thread = thread::spawn(|| {
        server::run_server().expect("Error running server");
    });
    client::run_client().expect("Error running client");
    server_thread.join().expect("Error joining server thread");
}
