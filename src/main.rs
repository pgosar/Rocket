use std::env::set_var;
mod client;
mod server;
mod test;
mod utils;
use test::run_seq::run;

#[tokio::main()]
async fn main() {
  set_var("RUST_BACKTRACE", "1");
  run().await;
}
