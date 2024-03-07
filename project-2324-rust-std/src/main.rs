use projet::{map_load::*, neighors::Neighbors};
use std::env;
pub mod map_loader;
use crate::map_loader::load_map;
use projet::find_neighbors::*;
use std::cmp::PartialEq;

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

    //find_neighbors test
    let neighbors_0 = Neighbors {
        nb: 2,
        neighbors_names: [2, 1, -1, -1],
        edges_cost: [0, -14, 0, 0],
    };
    let neighbors_1 = Neighbors {
        nb: 2,
        neighbors_names: [3, -1, -1, 0],
        edges_cost: [-19, 0, 0, -14],
    };
    let neighbors_2 = Neighbors {
        nb: 3,
        neighbors_names: [4, 3, 0, -1],
        edges_cost: [18, -8, 0, 0],
    };
    let neighbors_3 = Neighbors {
        nb: 3,
        neighbors_names: [5, -1, 1, 2],
        edges_cost: [0, 0, -19, -8],
    };
    let neighbors_4 = Neighbors {
        nb: 2,
        neighbors_names: [-1, 5, 2, -1],
        edges_cost: [0, -5, 18, 0],
    };
    let neighbors_5 = Neighbors {
        nb: 2,
        neighbors_names: [-1, -1, 3, 4],
        edges_cost: [0, 0, 0, -5],
    };

    print!("find_neighbors test :\n");

    assert_eq!(neighbors_0, find_neighbors(&test_map, 0));
    print!("Node 0 passed\n");
    assert_eq!(neighbors_1, find_neighbors(&test_map, 1));
    print!("Node 1 passed\n");
    assert_eq!(neighbors_2, find_neighbors(&test_map, 2));
    print!("Node 2 passed\n");
    assert_eq!(neighbors_3, find_neighbors(&test_map, 3));
    print!("Node 3 passed\n");
    assert_eq!(neighbors_4, find_neighbors(&test_map, 4));
    print!("Node 4 passed\n");
    assert_eq!(neighbors_5, find_neighbors(&test_map, 5));
    print!("Node 5 passed\n");

    println!("find_neighbors test passed");
}
