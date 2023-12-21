use base64::engine::general_purpose;
use base64::Engine;
use clap::{Arg, Command};
use getset::Getters;
use sha1::Digest;

pub const WEBSOCKET_PREFIX: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub fn sec_websocket_key(client_key: String) -> String {
  let combined = client_key + WEBSOCKET_PREFIX;
  let mut sha1 = sha1::Sha1::new();
  sha1.update(combined.as_bytes());
  let hash = sha1.finalize();
  let my_key: String = general_purpose::STANDARD.encode(&hash[..]);
  my_key
}

#[derive(Debug, Getters)]
pub struct Opts {
  #[getset(get = "pub")]
  threads: usize,
}

impl Opts {
  pub fn new() -> Self {
    let app = Command::new("Multithreaded Websocket Server")
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about("Control the websocket server")
      .arg(
        Arg::new("num_threads")
          .short('t')
          .long("num_threads")
          .value_name("NUM")
          .help("sets the number of threads")
          .required(false)
          .num_args(1),
      );
    let matches = app.get_matches();
    let num_cpus: &String = &std::thread::available_parallelism()
      .unwrap()
      .get()
      .to_string();
    let threads_str: &String = matches.get_one("num_threads").unwrap_or(num_cpus);
    let threads: usize = threads_str.parse::<usize>().unwrap();
    let opts = Opts { threads };
    opts
  }
}
