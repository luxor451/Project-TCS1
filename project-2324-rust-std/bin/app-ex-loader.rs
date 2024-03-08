//! A simple program to load a maze configuration file
//! and "print" it on the console
//!
//! Example of usage:
//!
//! cargo --bin app-ex-loader ./data/maze.txt
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
    let mut line = String::new();
    reader.read_line(&mut line)?;

    // read and print dimensions in the first line
    let mut width: usize = 0;
    let mut height: usize = 0;

    sscanf!(&line, "{} {}", width, height)?;

    println!("map dimensions: {width} x {height}");

    for node in 0..width * height {
        let mut line = String::new();
        let mut below: i32 = 0;
        let mut right: i32 = 0;
        reader.read_line(&mut line)?;
        sscanf!(&line, "{i32} {i32}", below, right)?;
        println!(
            "line {:4} -> {{ below: {below:3}, right: {right:3} }}",
            node + 2
        );
    }
    return Ok(());
}
