use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Print the program name and number of parameters
    println!("{} is called with {} parameters", args[0], args.len() - 1);

    // Print each parameter
    for (i, arg) in args.iter().enumerate().skip(1) {
        println!("parameter #{}: {}", i, arg);
    }
}