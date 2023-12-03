use std::env::set_var;
mod client;
mod run;
mod server;
mod utils;
use crate::utils::utils::Opts;
use run::run::run;

fn main() {
  set_var("RUST_BACKTRACE", "1");
  let opts: Opts = Opts::new();
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(*opts.threads())
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      run(opts).await;
    })
}
