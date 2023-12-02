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
  log: bool,
  #[getset(get = "pub")]
  verbosity: usize,
  #[getset(get = "pub")]
  repeats: u32,
  #[getset(get = "pub")]
  num_clients: usize,
  #[getset(get = "pub")]
  out_degree: usize,
  #[getset(get = "pub")]
  sleep_time_mean: f32,
  #[getset(get = "pub")]
  sleep_time_std: f32,
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
        Arg::new("debug")
          .short('d')
          .long("debug")
          .help("enables debugging mode")
          .required(false)
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
        Arg::new("log")
          .short('l')
          .long("log")
          .value_name("FILE")
          .help("sets whether to output to log file")
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
      )
      .arg(
        Arg::new("sleep_time")
          .short('s')
          .long("sleep_time")
          .value_name("NUM1, NUM2")
          .help("sets the mean and standard deviation of the sleep time between messages")
          .required(false)
          .default_value("1, 0")
          .num_args(2),
      )
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
    let debug = matches.contains_id("debug");
    let log = matches.contains_id("log");
    let verbosity_str: &String = matches.get_one("verbosity").unwrap();
    let verbosity: usize = verbosity_str.parse::<usize>().unwrap();
    let mode: &String = matches.get_one("mode").expect("mode is required");
    let repeats_str: &String = matches.get_one("repeats").unwrap();
    let repeats: u32 = repeats_str.parse::<u32>().unwrap();
    let num_clients_str: &String = matches.get_one("num_clients").unwrap();
    let num_clients: usize = num_clients_str.parse::<usize>().unwrap();
    let out_degree_str: &String = matches.get_one("out_degree").unwrap();
    let out_degree: usize = out_degree_str.parse::<usize>().unwrap();
    let num_cpus: &String = &std::thread::available_parallelism()
      .unwrap()
      .get()
      .to_string();
    let threads_str: &String = matches.get_one("num_threads").unwrap_or(num_cpus);
    let threads: usize = threads_str.parse::<usize>().unwrap();
    let sleep_time: Vec<String> = matches
      .get_many("sleep_time")
      .unwrap()
      .map(|v: &String| v.to_string())
      .collect();
    println!("{:?}", sleep_time);
    let sleep_time_mean: f32 = sleep_time[0].parse::<f32>().unwrap();
    let sleep_time_std: f32 = sleep_time[1].parse::<f32>().unwrap();
    let opts = Opts {
      mode: mode.to_string(),
      debug,
      log,
      verbosity,
      repeats,
      num_clients,
      out_degree,
      sleep_time_mean,
      sleep_time_std,
      threads,
    };
    if debug {
      println!("{:?}", opts);
    }
    opts
  }
}
