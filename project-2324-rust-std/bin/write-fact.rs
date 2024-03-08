//!
//! Simple program to explain how to write
//! text into a file
//!
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() -> io::Result<()> {
    let file = File::create("./data/fact.txt")?;
    {
        let mut writer = BufWriter::new(file);

        let mut fact = 1;

        for i in 0..10 {
            writeln!(writer, "{}! = {}", i, fact)?;
            fact = fact * (i + 1);
        }
    }
    return Ok(());
}
