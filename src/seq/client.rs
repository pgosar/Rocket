use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

struct Client {
    ip: String,
    port: u16,
}

pub fn run_client() -> std::io::Result<()> {
    std::thread::sleep(std::time::Duration::from_secs(1));
    let client = Client {
        ip: String::from("::1"),
        port: 8080,
    };
    let address: String = format!("[{}]:{}", client.ip, client.port);
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port {}", client.port);
            let msg = b"Hello World";

            let mut data = [0 as u8; 1024];
            stream.write(msg)?;
            println!("Sent Hello World message");
            match stream.read(&mut data) {
                Ok(_) => {
                    println!("Client Received: {}", from_utf8(&data).unwrap());
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
            stream.write(msg)?;
            println!("Sent Hello World message");
            let mut data = [0 as u8; 1024];
            match stream.read(&mut data) {
                Ok(_) => {
                    println!("Client Received: {}", from_utf8(&data).unwrap());
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            // the size becomes 0 in server.rs when this call finishes because
            // the connection closes when the listener scope is gone
            std::thread::sleep(std::time::Duration::from_secs(200));
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    Ok(())
}
