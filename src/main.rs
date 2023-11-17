use std::env::set_var;
mod seq;
mod utils;

fn main() {
    set_var("RUST_BACKTRACE", "1");
    seq::run();
}
