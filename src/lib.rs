use std::error::Error;
use std::fs;

use colored::{ColoredString, Colorize};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.fname)?;
    // println!("With contents:\n\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        // println!("{}", line);
        let words: Vec<&str> = line.split(' ').collect();
        for word in words {
            let mut found = ColoredString::from(word);
            if word.contains(&config.query) || (!config.case_sensitive
                && word.to_lowercase().contains(&config.query.to_lowercase())) {
                found = word.red();
            }
            print!("{} ", found);
        }
        print!("\n");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub fname: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let query = args[1].clone();
        let fname = args[2].clone();

        let case_sensitive = if args.len() == 4 {
            if !args[3].starts_with('-') || args[3] != "-s" {
                return Err("wrong flag");
            } else {
                true
            }
        } else {
            false
        };

        Ok(Config {
            query,
            fname,
            case_sensitive,
        })
    }
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
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
