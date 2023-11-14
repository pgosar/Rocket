use socket2::{Domain, Socket, Type};
use std::io::{prelude::*, BufReader};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
struct Server {
    ip: String,
    port: u32,
    key: String,
}

fn run() {
    let socket = Socket::new(Domain::IPV6, Type::STREAM, None).unwrap();
    socket.set_only_v6(false).unwrap();
    let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    socket.bind(&address.into());
    socket.listen(128).unwrap();
    let listener: TcpListener = socket.into();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let line = reader.lines().next().unwrap().unwrap();
    println!("{}", line);
}
