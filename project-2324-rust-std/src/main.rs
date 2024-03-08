use core::map_load::*;
use std::env;
pub mod map_loader;
use crate::map_loader::load_map;
pub mod core;
pub mod find_neighbors;

#[cfg(test)]
mod test {
    use crate::{
        core::{maze::Maze, neighors::Neighbors},
        find_neighbors::find_neighbors,
        map_loader::load_map,
    };
    #[test]
    fn test_find_neighbors() {
        let path_to_map1 = "./data/map_2_3_42.txt";
        let map_1 = load_map(path_to_map1.to_string());
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

        assert_eq!(neighbors_0, find_neighbors(&map_1, 0));
        print!("Node 0 passed\n");
        assert_eq!(neighbors_1, find_neighbors(&map_1, 1));
        print!("Node 1 passed\n");
        assert_eq!(neighbors_2, find_neighbors(&map_1, 2));
        print!("Node 2 passed\n");
        assert_eq!(neighbors_3, find_neighbors(&map_1, 3));
        print!("Node 3 passed\n");
        assert_eq!(neighbors_4, find_neighbors(&map_1, 4));
        print!("Node 4 passed\n");
        assert_eq!(neighbors_5, find_neighbors(&map_1, 5));
        print!("Node 5 passed\n");

        println!("find_neighbors test passed");
    }

    #[test]
    fn test_print_maze() {
        print!("print_maze test on maze_2048 :\n");
        let maze_array_2048: Vec<i32> = vec![
            -1, 0, 3, 13, 14, 4, 7, 17, 18, 19, 11, 1, 22, 12, 13, 14, 15, 18, 28, 29, 10, 20, 21,
            22, 34, 26, 16, 26, 38, 28, 20, 21, 31, 32, 44, 25, 37, 27, 37, 38, 50, 51, 32, 42, 43,
            46, 36, 37, 58, 59, 51, 52, 42, 63, 53, 56, 46, 47, 59, 69, 61, 62, 52, 73, 54, 64, 65,
            66, 67, 79, 60, 72, 62, 72, 75, 65, 66, 78, 68, 78,
        ];
        let maze_2048 = Maze {
            width: 10,
            height: 8,
            predecessor: maze_array_2048,
            cost: -666,
        };
        println!("{}", maze_2048);
    }
}

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
