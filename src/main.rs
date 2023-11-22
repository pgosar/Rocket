use std::env::set_var;
mod client;
mod server;
mod test;
mod utils;
use test::run_seq::run;
use crate::utils::utils::Opts;

#[tokio::main()]
async fn main() {
  let OPTS: Opts = Opts::new();
  set_var("RUST_BACKTRACE", "1");
  run(OPTS).await;
}
