use std::env::set_var;

mod seq;

fn main() {
  set_var("RUST_BACKTRACE", "1");
  seq::run();
}
