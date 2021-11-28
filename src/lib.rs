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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
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
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let case_query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&case_query) {
            result.push(line)
        }
    }
    result
}