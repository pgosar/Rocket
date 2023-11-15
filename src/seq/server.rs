use socket2::{Domain, Socket, Type};
use std::io::{prelude::*, BufReader};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};

struct Server {
    ip: String,
    port: u32,
    key: String,
}

pub fn run_server() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap(); //socket.into();
    println!("test");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
}
