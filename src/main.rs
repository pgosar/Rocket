use std::env::set_var;
mod client;
mod run;
mod server;
mod utils;
use crate::utils::utils::Opts;
use run::run_seq::run;

#[tokio::main()]
async fn main() {
  set_var("RUST_BACKTRACE", "1");
  let opts: Opts = Opts::new();
  run(opts).await;
}
