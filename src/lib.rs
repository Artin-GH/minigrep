use std::error::Error;
use std::{env, fs, io};

pub struct Config {
    query: String,
    file_path: Option<String>,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        let mut main_args: Vec<&String> = vec![];
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        for arg in args {
            if arg == "--ignore-case" {
                ignore_case = true
            } else {
                main_args.push(arg);
            }
        }
        if main_args.len() < 2 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: main_args[1].clone(),
            file_path: main_args.get(2).map(|s| (*s).clone()),
            ignore_case,
        })
    }
}

fn search_case_sensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<&'a str>>()
}

fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| {
            line.to_ascii_lowercase()
                .contains(&query.to_ascii_lowercase())
        })
        .collect::<Vec<&'a str>>()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = if let Some(path) = &config.file_path {
        fs::read_to_string(path)
    } else {
        io::read_to_string(io::stdin())
    }?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let text = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, text)
        );
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
