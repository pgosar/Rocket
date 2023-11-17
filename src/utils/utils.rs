use crate::utils::logging;
use std::io::Write;
use std::net::TcpStream;

pub enum ErrorLevel {
    INFO,
    WARNING,
    ERROR,
}

pub struct Message {
    msg: String,
    error_level: ErrorLevel,
    timestamp: f64,
}

impl Message {
    pub fn get_timestamp(&self) -> f64 {
        self.timestamp
    }
}

pub fn send_message(m: Message, mut stream: TcpStream) {
    stream.write(m.msg.as_bytes()).unwrap();
    // TODO: log msg
}
