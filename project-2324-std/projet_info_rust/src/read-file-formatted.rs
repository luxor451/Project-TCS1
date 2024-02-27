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

    let mut line_nb = 1;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Error reading line {}!", line_nb);
                std::process::exit(1);
            }
        };

        let mut parts = line.trim().split(" - ");
        let first_int: i32 = match parts.next().unwrap().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Line number {} is not syntactically correct!", line_nb);
                std::process::exit(1);
            }
        };
        let second_int: i32 = match parts.next().unwrap().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Line number {} is not syntactically correct!", line_nb);
                std::process::exit(1);
            }
        };

        println!("first value at line {}: {}", line_nb, first_int);
        println!("second value at line {}: {}", line_nb, second_int);

        line_nb += 1;
    }
}