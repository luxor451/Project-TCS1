use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // testing if a filename has been given
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("You must provide a file name!");
        std::process::exit(1);
    }

    // open file
    let file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Cannot read file {}!", &args[1]);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    // read dimensions on first line
    let mut width = 0;
    let mut height = 0;
    let mut lines = reader.lines();

    if let Some(Ok(line)) = lines.next() {
        let mut parts = line.trim().split_whitespace();
        width = parts.next().unwrap().parse().unwrap();
        height = parts.next().unwrap().parse().unwrap();
    } else {
        eprintln!("First line is not syntactically correct!");
        std::process::exit(1);
    }

    println!("map dimensions: {} x {}", width, height);

    // read following lines
    // each line is composed of two integer values
    for (i, line) in lines.enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Error reading line {}", i + 2);
                std::process::exit(1);
            }
        };

        let mut parts = line.trim().split_whitespace();
        let below: i32 = match parts.next().unwrap().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Line {} is not syntactically correct!", i + 2);
                std::process::exit(1);
            }
        };
        let right: i32 = match parts.next().unwrap().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Line {} is not syntactically correct!", i + 2);
                std::process::exit(1);
            }
        };

        println!("line {:4}-> {{ below: {}, right: {} }}", i + 2, below, right);
    }
}