use std::env;
use std::process;
use minigrep::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for: {}\nIn File: {}",config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}