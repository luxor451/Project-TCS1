//!
//! Simple program to explain how to create
//! strings with variable values
//!

use std::io::Write;

fn main() {
    let original = "simulation";
    let length = original.len();

    // declaring a "string" of capacity length + 6 characters
    // indeed, any capacity may be specified
    // since Vec is dynamically resizable
    let mut my_string: Vec<u8> = Vec::with_capacity(length + 6);
    write!(my_string, "simulation-{}.{}", 3, "txt").expect("bad things happened !");

    // my_string is not really a String but can be converted
    println!("The new string: {}", String::from_utf8_lossy(&my_string));
}
