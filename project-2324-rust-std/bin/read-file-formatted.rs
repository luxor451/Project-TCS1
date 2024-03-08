//!
//! Simple program to explain how to read
//! formatted data from a file
//!
use scanf::sscanf;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "You must provide a file name");

    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);

    let mut first_int: i32 = 0;
    let mut second_int: i32 = 0;
    let mut line_nb: i32 = 0;

    let mut line = String::new();
    reader.read_line(&mut line)?;
    sscanf!(&line, "{} {}", first_int, second_int)?;

    loop {
        println!("first value at line {line_nb}: {first_int}");
        println!("second value at line {line_nb}: {second_int}");

        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        sscanf!(&line, "{} {}", first_int, second_int)?;
        line_nb += 1;
    }

    return Ok(());
}
