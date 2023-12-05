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
  my_id: u32,
  #[getset(get = "pub")]
  debug: bool,
  #[getset(get = "pub")]
  verbosity: usize,
  #[getset(get = "pub")]
  repeats: u32,
  #[getset(get = "pub")]
  num_clients: usize,
  #[getset(get = "pub")]
  out_degree: usize,
  #[getset(get = "pub")]
  sleep_time_mean: u32,
  #[getset(get = "pub")]
  output_path: String,
  #[getset(get = "pub")]
  message_length: u32,
}

impl Opts {
  pub fn new() -> Self {
    let app = Command::new("Websocket Client")
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about("Control the websocket client")
      .arg(
        Arg::new("my_id")
          .short('i')
          .long("my_id")
          .help("specify what to tell the server your ID is")
          .required(false)
          .default_value("0")
          .num_args(1),
      )
      .arg(
        Arg::new("debug")
          .short('d')
          .long("debug")
          .help("enables debugging mode")
          .required(false)
          .action(clap::ArgAction::SetTrue)
          .num_args(0),
      )
      .arg(
        Arg::new("verbosity")
          .short('v')
          .long("verbose")
          .help("sets the level of verbosity for debugging output")
          .required(false)
          .value_parser(["0", "1", "2", "3"])
          .default_value("0")
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
          .help("sets the number of clients you send message to")
          .required(false)
          .default_value("1")
          .num_args(1),
      )
      .arg(
        Arg::new("sleep_time")
          .short('s')
          .long("sleep_time")
          .value_name("NUM")
          .help("sets the sleep time between messages")
          .required(false)
          .default_value("1")
          .num_args(1),
      )
      .arg(
        Arg::new("output_path")
          .short('f')
          .long("output_path")
          .value_name("STRING")
          .help("Specifies path ")
          .required(false)
          .default_value("log.txt")
          .num_args(1),
      )
      .arg(
        Arg::new("message_length")
          .short('m')
          .long("message_length")
          .value_name("NUM")
          .help("Length of the client messages")
          .required(false)
          .default_value("10")
          .num_args(1),
      );

    let matches = app.get_matches();
    let my_id_str: &String = matches.get_one("my_id").unwrap();
    let my_id = my_id_str.parse::<u32>().unwrap();
    let debug = matches.get_flag("debug");
    let verbosity_str: &String = matches.get_one("verbosity").unwrap();
    let verbosity: usize = verbosity_str.parse::<usize>().unwrap();
    let repeats_str: &String = matches.get_one("repeats").unwrap();
    let repeats: u32 = repeats_str.parse::<u32>().unwrap();
    let num_clients_str: &String = matches.get_one("num_clients").unwrap();
    let num_clients: usize = num_clients_str.parse::<usize>().unwrap();
    let out_degree_str: &String = matches.get_one("out_degree").unwrap();
    let out_degree: usize = out_degree_str.parse::<usize>().unwrap();
    let sleep_time_str: &String = matches.get_one("sleep_time").unwrap();
    let sleep_time_mean: u32 = sleep_time_str.parse::<u32>().unwrap();
    let output_path: &String = matches.get_one("output_path").unwrap();
    let message_length_str: &String = matches.get_one("message_length").unwrap();
    let message_length: u32 = message_length_str.parse::<u32>().unwrap();
    let opts = Opts {
      my_id,
      debug,
      verbosity,
      repeats,
      num_clients,
      out_degree,
      sleep_time_mean,
      output_path: output_path.clone(),
      message_length,
    };
    if debug {
      println!("{:?}", opts);
    }
    opts
  }
}
