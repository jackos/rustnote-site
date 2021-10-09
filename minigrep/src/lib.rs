use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}",contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        Ok(Config{query: args[1].clone(), filename: args[2].clone()})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
println!("{}", contents);
    }
}