/* //! An integration test file.
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use projet::fact;
use scanf::sscanf;

#[cfg(test)]
fn test_fact() {
    assert_eq!(fact(2), 2);
}

fn file_fact() -> io::Result<()> {
    let file = File::open("./data/fact.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);

    // read and print dimensions in the first line

    for _ in 0..10 {
        let mut line = String::new();
        let mut f: i32 = 0;
        let mut n: i32 = 0;

        reader.read_line(&mut line)?;
        sscanf!(&line, "{i32}! = {i32}", n, f)?;
        assert_eq!(f, fact(n));
    }
    return Ok(());
}
 */