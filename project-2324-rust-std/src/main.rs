//! This project, since it is in Rust, does not respect completly the struture given in the subject
//! To use prim's algorithme, you should use the main i.e. run "cargo run --bin projet -- filename"
//! This will display in the terminal the maze generated with prim_naive
//!
//! For the tests use "cargo test"
//!
//! In core.rs is implemented all the different struct with their implementation and formating functions
//! In find_neighors is implemented the find_neighbors function but most of it is the module Neighbors

use core::map_load::*;
use std::env;
pub mod map_loader;
mod prim_naive;
mod tests;
use crate::{core::maze::Maze, map_loader::load_map, prim_naive::prim_naive_function};
pub mod core;
pub mod find_neighbors;

fn main() {
    //read file name from command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a filename");
    }
    let filename: &String = &args[1];

    //Matrix test
    /* println!("\nMatrix test :\n");
    let mut mv: Matrix<i32> = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    *mv.get_mut(1, 2) = 10;
    println!("Display:\n {}", mv);
    println!("Debug: \n{:?}", mv); */
    //load_map test
    /* println!("\nload_map test :\n"); */
    let test_map: Map = load_map(filename.to_string());
    /* println!("Display:\n {}", test_map.edges_matrix); */
    let prim: (Vec<i32>, i32) = prim_naive_function(&test_map);
    // Get the dimentions from the filename
    let numbers: Vec<&str> = filename.split('_').collect();
    let maze_2048: Maze = Maze {
        width: numbers[1].parse::<usize>().ok().unwrap(),
        height: numbers[2].parse::<usize>().ok().unwrap(),
        predecessor: prim.0,
        cost: prim.1,
    };
    println!("\n{} \n ^^ Maze generated with prim_naive", maze_2048);
    let _ = Maze::write_maze_in_pbm(&maze_2048);
}
