//!
//! Simple program to explain how to read
//! text lines from a file
//!
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

    let mut line: i32 = 0;

    // while not having encountered end of file, read lines
    loop {
        // declaration of a new buffer
        // to which read chars will be appended
        let mut buffer = String::new();
        // read lines until 0 bytes can be read
        // i.e. end of file has been met
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }
        line += 1;
        println!("{line:4}: {buffer}");
    }

    return Ok(());
}
