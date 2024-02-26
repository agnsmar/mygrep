use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_content = fs::read_to_string(&config.file_path)?;
  
  println!("Searching for '{}'", config.query);
  println!("In file '{}'", config.file_path);
  println!("File Content:\n{file_content}");

  Ok(())
}

pub struct Config {
  query: String,
  file_path: String
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() != 3 {
          return Err("incorrect number of arguments")
      }

      let query = args[1].clone();
      let file_path = args[2].clone();
  
      Ok(Config { query, file_path })
  }
}