use std::env;
use std::process;

use mygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();

    if let Err(e) = mygrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
