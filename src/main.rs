use std::env::set_var;
mod client;
mod server;
mod test;
mod utils;
use crate::utils::utils::Opts;
use test::run_seq::run;

#[tokio::main()]
async fn main() {
  let opts: Opts = Opts::new();
  set_var("RUST_BACKTRACE", "1");
  run(opts).await;
}
