use crate::utils;
use std::collections::VecDeque;
use std::time::SystemTime;

pub struct Logger {
    entries: VecDeque<utils::Message>,
    max_time: f64,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            entries: VecDeque::new(),
            max_time: std::time::Duration::from_secs(10 * 60).as_secs_f64(), // 10 minutes
        }
    }

    pub fn log(&mut self, m: utils::Message) {
        self.entries.push_back(m);
        let cur_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let mut first_entry = self.entries.front().unwrap();
        while first_entry.get_timestamp() < cur_time - self.max_time {
            self.entries.pop_front();
            first_entry = self.entries.front().unwrap();
        }
    }
}
