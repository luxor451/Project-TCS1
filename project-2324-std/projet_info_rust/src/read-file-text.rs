use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Cannot read file {}!", &args[1]);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    let mut line = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(l) => {
                line += 1;
                println!("{:2}: {}", line, l);
            }
            Err(_) => {
                eprintln!("Error reading line {}!", line + 1);
                std::process::exit(1);
            }
        }
    }
}