//minigrep
//simple grep implementation in Rust
//Learning project by Mark Decker
//11/11/2024
//
use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    //Parse the input arguments
    let config = Config::build(&args[..]).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Runtime error: {e}");
        process::exit(2);
    }
}

