use colored::{ColoredString, Colorize};
use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for mut line in results {
        line.print();
    }

    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("incorrect number of arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub struct Line {
    line_number: i32,
    words: Vec<ColoredString>,
}

impl Line {
    fn new(line_number: i32, words: Vec<ColoredString>) -> Line {
        Line { line_number, words }
    }

    fn print(&mut self) -> () {
        print!("{}: ", self.line_number);
        for word in &self.words {
            print!("{word} ");
        }
        println!()
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<Line> {
    let mut results = Vec::new();

    let mut count = 1;
    for line in contents.lines() {
        if line.contains(query) {
            let words: Vec<&str> = line.split(" ").collect();
            let mut result: Vec<ColoredString> = Vec::new();
            for word in words {
                if word.contains(query) {
                    result.push(word.green().bold().underline());
                } else {
                    result.push(word.white());
                }
            }
            results.push(Line::new(count, result));
        }
        count += 1;
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Line> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    let mut count = 1;
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            let words: Vec<&str> = line.split(" ").collect();
            let mut result: Vec<ColoredString> = Vec::new();
            for word in words {
                if word.to_lowercase().contains(&query) {
                    result.push(word.green().bold().underline());
                } else {
                    result.push(word.white());
                }
            }
            results.push(Line::new(count, result));
        }
        count += 1;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
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
