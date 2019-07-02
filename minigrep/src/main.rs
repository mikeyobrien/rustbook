extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

/*
Roles of main
- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error
*/

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
