use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.file_path)?;
    
    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);
    println!("File Content:\n{file_content}");

    Ok(())
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


