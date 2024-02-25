use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    let file_content = fs::read_to_string(file_path)
        .expect("Can't read file content");

    println!("Searching for '{}'", query);
    println!("In file '{}'", file_path);
    println!("File Content:\n{}", file_content);
}
