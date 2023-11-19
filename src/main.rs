use std::env::set_var;
mod test;
mod utils;
mod server;
mod client;
use test::run_seq::run;

fn main() {
  set_var("RUST_BACKTRACE", "1");
  run();
}
