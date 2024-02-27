use std::fs::File;
use std::io::Write;

fn main() {
    // Open or create the file named "fact.txt" for writing
    let mut p_file = match File::create("fact.txt") {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Cannot write to file fact.txt!");
            std::process::exit(1);
        }
    };

    let mut fact = 1;

    // Write factorial values to the file
    for i in 0..10 {
        writeln!(p_file, "{}! = {}", i, fact).expect("Error writing to file");

        fact *= i + 1;
    }

    // Close the file
    drop(p_file);
}