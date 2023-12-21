use std::env::set_var;
mod run;
mod server;
mod utils;
use crate::utils::utils::Opts;
use run::run::run;
use tracing::{info, Level};

fn main() {
  set_var("RUST_BACKTRACE", "1");
  tracing_subscriber::fmt()
    .with_max_level(Level::TRACE)
    .init();
  let opts: Opts = Opts::new();
  info!("Starting server with options: {:?}", opts);
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(*opts.threads())
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      run().await;
    })
}
