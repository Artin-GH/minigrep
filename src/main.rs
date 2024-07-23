use minigrep::Config;
use std::{process, env};


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(err) = minigrep::run(config) {
        eprintln!("Error: {}", err);
        process::exit(1);
    };
}
