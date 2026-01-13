use std::{env, fs, process};

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() < 2 {
        println!("No file specified!");
        process::exit(1);
    }

    let exists = fs::exists(&args[1]);
    if exists.is_err() || !exists.unwrap() {
        println!("Invalid file path");
        process::exit(1);
    }

    let file_path = &args[1];
    let token_list = parser::parse_file(file_path);

    dbg!(token_list);
}
