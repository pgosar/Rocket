use clap::{Arg, Command};
use getset::Getters;

pub const OPTS: Opts = None

#[derive(Debug, Getters)]
pub struct Opts {
  #[getset(get = "pub")]
  mode: String,
  #[getset(get = "pub")]
  debug: bool,
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
      );
    let matches = app.get_matches();
    let debug = matches.contains_id("debug");
    let mode: &String = matches.get_one("mode").unwrap();
    Opts {
      mode: mode.to_string(),
      debug,
    }
  }
}
