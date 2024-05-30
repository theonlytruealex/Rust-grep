use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;
mod words;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut case_sensitive: i32 = 0;
    let mut is_word: i32 = 1;
    if args.len() < 3 {
        println!("Provide a file and a search-word");
        process::exit(1);
    }
    if args.len() >= 4 {
        if args[3] == "cased" {
            case_sensitive = 1;
        }
    }
    if args.len() >= 5 {
        if args[4] == "string" {
            is_word = 0;
        }
    }
    let raw_word = &args[2];
    if !raw_word.chars().all(char::is_alphanumeric) {
        is_word = 0;
    }
    let path = Path::new(&args[1]);
    if !path.is_file() {
        println!("First argument not a file");
        process::exit(1);
    }
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            process::exit(1);
        }
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if is_word == 1 {
                    if case_sensitive == 0 {
                        words::not_case_sensitive(&line, &raw_word);
                    } else {
                        words::case_sensitive(&line, &raw_word);
                    }
                } else if line.contains(raw_word) {
                    println!("{}", line);
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                process::exit(1);
            }
        }
    }
}
