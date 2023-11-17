use crate::utils::logging;
use std::io::Write;
use std::net::TcpStream;

pub fn send_message(m: logging::Message, mut stream: TcpStream) {
    stream.write(m.msg.as_bytes()).unwrap();
    // TODO: log msg
}
