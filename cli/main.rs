use xo_runtime as xo;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  println!("{}", xo::run_file(&args[1], &xo::RuntimeOptions::default()));
}
