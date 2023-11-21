use std::env::set_var;
mod client;
mod server;
mod test;
mod utils;
use pollster::block_on;
use test::run_seq::run;

fn main() {
  set_var("RUST_BACKTRACE", "1");
  block_on(run());
}
