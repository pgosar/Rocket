use clap::{Arg, Command};
use getset::Getters;

#[derive(Debug, Getters)]
pub struct Opts {
  #[getset(get = "pub")]
  mode: String,
  #[getset(get = "pub")]
  debug: bool,
  #[getset(get = "pub")]
  repeats: i32,
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
      );
    let matches = app.get_matches();
    let debug = matches.contains_id("debug");
    let mode: &String = matches.get_one("mode").expect("mode is required");
    let default: &String = &String::from("1");
    let repeats_str: &String = matches.get_one("repeats").unwrap_or(default);
    let repeats: i32 = repeats_str.parse::<i32>().unwrap();
    println!("mode: {} debug: {} repeats: {}", mode, debug, repeats);
    Opts {
      mode: mode.to_string(),
      debug,
      repeats,
    }
  }
}
