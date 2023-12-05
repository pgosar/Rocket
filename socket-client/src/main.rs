pub mod testclient;
pub mod clientsocket;
mod utils;
use utils::Opts;
use std::env::set_var;
use fs2::FileExt;
use std::fs::OpenOptions;
use std::io::Write;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub async fn run(opts: Opts) {
  let i = *opts.my_id();
  let debug = *opts.debug();
  let repeats = *opts.repeats();
  let out_degree = *opts.out_degree() as usize;
  let num_clients = *opts.num_clients();
  let sleep_mean: u32 = *opts.sleep_time_mean();
  let output_path: String = opts.output_path().clone();
  let message_length: usize = *opts.message_length() as usize;
  let sleep_padding: u32 = 2000;
  let total_subtracted = std::time::Duration::from_millis((sleep_padding * 2 + sleep_mean * repeats).into());
  let mut my_client =
    testclient::TestClient::new(String::from("localhost:8080"), i, debug);
  let start = std::time::Instant::now();
  let rng = thread_rng();
  let random_msg: String = rng.sample_iter(&Alphanumeric).take(message_length).map(char::from).collect();
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

  let end = std::time::Instant::now();
  // end to end runtime - client construction time - client sleep times
  let total_time = end
    .duration_since(start)
    .checked_sub(total_subtracted)
    .unwrap();
  let mut file = OpenOptions::new().append(true).create(true).open(output_path).expect("error opening file");
  file.lock_exclusive().unwrap();
  let nanos = (total_time.as_nanos() as f64) / 1000000.0;
  writeln!(file, "{},{},{},{},{:?}", num_clients, repeats, out_degree, sleep_mean, nanos).expect("error writing to file");
  file.unlock().unwrap();
}

pub fn main() {
  set_var("RUST_BACKTRACE", "1");
  let opts: Opts = Opts::new();
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      run(opts).await;
    })
}
