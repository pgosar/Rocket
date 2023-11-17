use std::collections::VecDeque;
use std::time::SystemTime;

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
  pub fn new(msg: String, error_level: ErrorLevel) -> Message {
    Message {
      msg,
      error_level,
      timestamp: SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64(),
    }
  }
}

pub struct Logger {
  entries: VecDeque<Message>,
  max_time: f64,
}

impl Logger {
  pub fn new() -> Logger {
    Logger {
      entries: VecDeque::new(),
      max_time: std::time::Duration::from_secs(10 * 60).as_secs_f64(), // 10 minutes
    }
  }

  pub fn print_log(&self) {
    for entry in &self.entries {
      match entry.error_level {
        ErrorLevel::INFO => println!("[INFO] {}", entry.msg),
        ErrorLevel::WARNING => println!("[WARNING] {}", entry.msg),
        ErrorLevel::ERROR => println!("[ERROR] {}", entry.msg),
      }
    }
  }

  pub fn log(&mut self, m: Message) {
    self.entries.push_back(m);
    let cur_time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs_f64();
    let mut first_entry = self.entries.front().unwrap();
    while first_entry.timestamp < cur_time - self.max_time {
      self.entries.pop_front();
      first_entry = self.entries.front().unwrap();
    }
  }
}
