use std::fs;
use std::error::Error;
use std::vec::Vec;
use std::env;

pub struct Config {
    file_name: String,
    query: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn`t get query string"),
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn`t get file name"),
        };
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { file_name, query, case_insensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file_content = fs::read_to_string(config.file_name)?;
    let result = if config.case_insensitive {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn onex_result() {
        let query = "rust";
        let content = "\
        rust
        ";
        assert_eq!(vec!["rust"], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let content = "\
        Rust
        ";
        assert_eq!(vec!["Rust"], search_case_insensitive(query, content))
    }
}

fn search<'a>(query: &str, content: &'a str) ->Vec<&'a str> {
    content.lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
    .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
    .collect()
}