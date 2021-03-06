//! # Minigrep
//!
//! `minigrep` is a light version of the popular command-line utility `grep`

use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// Runs the program using the config and search functions.
///
/// Performs the following operations:
/// - reading from a given filename
/// - searches for the given query with the appropriate search function
/// - prints the results found
/// - returns `Ok(())` if successful
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

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

/// Searches the `query` in the `contents` given - case sensitive.
/// Returns a vector of string slices representing the lines where the query is found.
///
/// # Examples
///
/// ```
/// let query = "the";
/// let contents = "How public, like The Frog\nTo tell your name the livelong day";
/// let result = vec!["To tell your name the livelong day"];
///
/// assert_eq!(result, minigrep_ag::search(query, contents))
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches the `query` in the `contents` given - case insensitives.
/// Returns a vector of string slices representing the lines where the query is found.
///
/// # Examples
///
/// ```
/// let query = "the";
/// let contents = "How public, like The Frog\nTo tell your name the livelong day";
/// let result = vec!["How public, like The Frog", "To tell your name the livelong day"];
///
/// assert_eq!(result, minigrep_ag::search_case_insensitive(query, contents))
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
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
