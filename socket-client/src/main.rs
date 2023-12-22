pub mod clientsocket;
pub mod testclient;
mod utils;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env::set_var;
use tracing::info;
use utils::Opts;

pub async fn run(opts: Opts) {
  let i = *opts.my_id();
  let repeats = *opts.repeats();
  let out_degree = *opts.out_degree() as usize;
  let num_clients = *opts.num_clients();
  let sleep_mean: u32 = *opts.sleep_time_mean();
  let message_length: usize = *opts.message_length() as usize;
  let sleep_padding: u32 = 2000;
  let mut my_client = testclient::TestClient::new(String::from("localhost:8080"), i);
  let rng = thread_rng();
  let random_msg: String = rng
    .sample_iter(&Alphanumeric)
    .take(message_length)
    .map(char::from)
    .collect();
  my_client
    .run_client(
      random_msg.to_lowercase(),
      repeats,
      num_clients,
      out_degree,
      sleep_mean,
      sleep_padding as u64,
    )
    .await
    .unwrap();
}

pub fn main() {
  set_var("RUST_BACKTRACE", "1");
  let opts: Opts = Opts::new();
  info!("Running with options: {:?}", opts);
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      run(opts).await;
    })
}
