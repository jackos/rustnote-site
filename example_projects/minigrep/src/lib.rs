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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower = &query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.contains(query_lower))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        args.next();
        let query = args.next().expect("No query provided");
        let filename = args.next().expect("No filename provided");

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
