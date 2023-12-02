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
  mode: String,
  #[getset(get = "pub")]
  debug: bool,
  #[getset(get = "pub")]
  repeats: i32,
  #[getset(get = "pub")]
  num_clients: i32,
  #[getset(get = "pub")]
  out_degree: i32,

}

impl Opts {
  pub fn new() -> Self {
    let app = Command::new("Multithreaded Websocket Server")
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about("Control the websocket server")
      .arg(
        Arg::new("debug")
          .short('d')
          .long("debug")
          .help("enables debugging mode")
          .required(false)
          .num_args(0),
      )
      .arg(
        Arg::new("mode")
          .short('m')
          .long("mode")
          .value_name("MODE")
          .help("sets the server mode")
          .value_parser(["c", "s"])
          .required(true)
          .num_args(1),
      )
      .arg(
        Arg::new("repeats")
          .short('r')
          .long("repeat")
          .value_name("NUM")
          .help("sets the number of repeat messages")
          .required(false)
          .default_value("1")
          .num_args(1),
      )
      .arg(
        Arg::new("num_clients")
          .short('n')
          .long("num_clients")
          .value_name("NUM")
          .help("sets the number of clients")
          .required(false)
          .default_value("1")
          .num_args(1),
      )
      .arg(
        Arg::new("out_degree")
          .short('o')
          .long("out_degree")
          .value_name("NUM")
          .help("sets the number of clients")
          .required(false)
          .default_value("1")
          .num_args(1),
      );
    let matches = app.get_matches();
    let debug = matches.contains_id("debug");
    let mode: &String = matches.get_one("mode").expect("mode is required");
    let default: &String = &String::from("1");
    let repeats_str: &String = matches.get_one("repeats").unwrap_or(default);
    let repeats: i32 = repeats_str.parse::<i32>().unwrap();
    let num_clients_str: &String = matches.get_one("num_clients").unwrap_or(default);
    let num_clients: i32 = num_clients_str.parse::<i32>().unwrap();
    let out_degree_str: &String = matches.get_one("out_degree").unwrap_or(default);
    let out_degree: i32 = out_degree_str.parse::<i32>().unwrap();
    println!("mode: {} debug: {} repeats: {}", mode, debug, repeats);
    Opts {
      mode: mode.to_string(),
      debug,
      repeats,
      num_clients,
      out_degree,
    }
  }
}
