use std::env;
use std::process;

use ugrep::Config;

fn main() {
	let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching in file {} for {} :", config.filename, config.query);
	
	if let Err(e) = ugrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}