use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Error parsing arguments: {e}");
        process::exit(1)
    });

    run(config)
}

fn run(config: Config) -> () {
    let file_content = fs::read_to_string(&config.file_path)
        .expect("Can't read file content");

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);
    println!("File Content:\n{}", file_content);
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("incorrect number of arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}


