extern crate rusty_vm;

use std::env;
use std::process;

use rusty_vm::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argumets: {}", err);
        process::exit(1);
    });

    if let Err(e) = rusty_vm::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
