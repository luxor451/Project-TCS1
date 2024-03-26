//! This project, since it is in Rust, does not respect completly the struture given in the subject
//! To use prim's algorithme, you should use the main i.e. run "cargo run --bin projet -- filename"
//! This will display in the terminal the maze generated with prim_naive
//!
//! For the tests use "cargo test"
//!
//! In core.rs is implemented all the different struct with their implementation and formating functions
//! In find_neighors is implemented the find_neighbors function but most of it is the module Neighbors

use core::map_load::*;
use std::{env, time::Instant};
pub mod map_loader;
mod prim_bh;
mod prim_naive;
mod tests;
use crate::{
    core::maze::Maze, map_loader::load_map, prim_bh::prim_bh_funtion,
    prim_naive::prim_naive_function,
};
pub mod core;
pub mod find_neighbors;

fn main() {
    //read file name from command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a filename");
    }
    let filename: &String = &args[1];

    let test_map: Map = load_map(filename.to_string());
    /* println!("Display:\n {}", test_map.edges_matrix); */
    let start = Instant::now();
    let prim: (Vec<i64>, i64) = prim_naive_function(&test_map);
    let duration = start.elapsed();
    println!("Time elapsed in prim_naive_function is: {:?}", duration);
    // Get the dimentions from the filename
    let numbers: Vec<&str> = filename.split('_').collect();
    let _maze_2048: Maze = Maze {
        width: numbers[1].parse::<usize>().ok().unwrap(),
        height: numbers[2].parse::<usize>().ok().unwrap(),
        predecessor: prim.0,
        cost: prim.1,
    }; 
    // println!("\n{} \n ^^ Maze generated with prim_naive", maze_2048);
    let start = Instant::now();
    let prim_bh: (Vec<i64>, i64) = prim_bh_funtion(&test_map);
    let duration = start.elapsed();
    println!("Time elapsed in prim_bh_funtion is: {:?}", duration);
    let numbers: Vec<&str> = filename.split('_').collect();
    let maze_2048: Maze = Maze {
        width: numbers[1].parse::<usize>().ok().unwrap(),
        height: numbers[2].parse::<usize>().ok().unwrap(),
        predecessor: prim_bh.0,
        cost: prim_bh.1,
    };
    // println!("\n{} \n ^^ Maze generated with prim_bh", maze_2048); 
    let _ = Maze::write_maze_in_pbm(&maze_2048);
}
