use std::env;
use std::error::Error;
use std::fs;

// Defines acceptable types of argument user can input.
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    // Result enum is a I am prepared for this error.
    // 'static indicates a lifetime for the whole program.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Requires two arguments, query and ./file/path");
        }
        // pass a reference to the slice object,
        // Clone the values found at the slice object's index
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE__CASE").is_ok();

        return Ok(Config {
            query,
            file_path,
            ignore_case,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}")
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // The lifetime annotations of the arguments idicate that the output
    // should live as long as the input of contents.

    // make mutable vector to append the results into.
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    return res;
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
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
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
