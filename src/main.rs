use std::env::set_var;
mod client;
mod run;
mod server;
mod utils;
use crate::utils::utils::Opts;
use run::run_seq::run;

#[tokio::main()]
async fn main() {
  let opts: Opts = Opts::new();
  set_var("RUST_BACKTRACE", "1");
  run(opts).await;
}
