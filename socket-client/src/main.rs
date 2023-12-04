pub mod testclient;
pub mod clientsocket;
mod utils;
use utils::Opts;
use std::env::set_var;
use fs2::FileExt;
use std::fs::OpenOptions;
use std::io::Write;


pub async fn run(opts: Opts) {
  let i = *opts.my_id();
  let debug = *opts.debug();
  let repeats = *opts.repeats();
  let out_degree = *opts.out_degree() as usize;
  let num_clients = *opts.num_clients();
  let sleep_mean: u32 = *opts.sleep_time_mean();
  let output_path: String = opts.output_path().clone();
  let total_subtracted = std::time::Duration::from_millis((4000 + sleep_mean * repeats).into());
  let mut my_client =
    testclient::TestClient::new(String::from("localhost:8080"), i, debug);
  let start = std::time::Instant::now();
  my_client
    .run_client(
        String::from("Hello World"),
        repeats,
        num_clients,
        out_degree,
        sleep_mean,
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
  writeln!(file, "{:?}", total_time.as_nanos()).expect("error writing to file");
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
