use std::env;
use std::error::Error;
use std::fs::read_to_string;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = &query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn on_result() {
        let mut query = "duct";
        let contents = read_to_string("src/tests/on_result.txt").expect("Can't read file");
        assert_eq!(vec!["safe, fast, productive"], search(query, &contents));
        query = "notEq";
        assert_ne!(vec!["random text"], search(query, &contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = read_to_string("src/tests/case.txt").expect("Can't read file");
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, &contents)
        );
    }
}
