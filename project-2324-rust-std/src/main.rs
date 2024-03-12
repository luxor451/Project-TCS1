use core::map_load::*;
use std::env;
pub mod map_loader;
mod prim_naive;
mod tests;
use crate::map_loader::load_map;
pub mod core;
pub mod find_neighbors;

fn main() {
    //read file name from command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a filename");
    }
    //Matrix test
    println!("\nMatrix test :\n");
    let filename = &args[1];
    let mut mv = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    *mv.get_mut(1, 2) = 10;
    println!("Display:\n {}", mv);
    println!("Debug: \n{:?}", mv);
    //load_map test
    println!("\nload_map test :\n");
    let test_map = load_map(filename.to_string());
    println!("Display:\n {}", test_map.edges_matrix);
}
