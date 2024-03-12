use core::map_load::*;
use std::env;
pub mod map_loader;
mod prim_naive;
use crate::map_loader::load_map;
pub mod core;
pub mod find_neighbors;



#[cfg(test)]
mod test {
    use crate::{
        core::{maze::*, neighors::Neighbors},
        find_neighbors::find_neighbors,
        map_loader::load_map, prim_naive::prim_naive_function,
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

    #[test]
    fn test_maze_in_pbm() {
        
        print!("pbm test on maze_2048 :\n");
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
        let _ = Maze::write_maze_in_pbm(&maze_2048);
    }
    #[test]
    fn test_prim_naif() {
        /* let maze_array_42 = vec![
            -1, 0, 1, 2, 3, 4, 16, 17, 18, 8, 0, 1, 11, 12, 13, 14, 15, 27, 28, 9, 30, 31, 21, 24, 14, 24,
            25, 26, 27, 19, 31, 41, 33, 23, 33, 34, 35, 27, 28, 29, 50, 42, 32, 33, 45, 46, 36, 37, 38, 39,
            51, 41, 42, 52, 53, 65, 55, 47, 57, 58, 50, 62, 72, 53, 63, 64, 65, 66, 69, 79, 60, 70, 73, 63,
            64, 76, 77, 67, 77, 78,
        ];  */
        /* let maze_array_76 = vec![
            -1, 2, 3, 4, 5, 15, 16, 17, 18, 8, 0, 10, 13, 14, 4, 25, 26, 16, 17, 18, 30, 11, 21, 22, 23, 24,
            36, 17, 18, 19, 31, 21, 31, 34, 35, 25, 35, 36, 39, 29, 30, 51, 52, 42, 43, 44, 45, 48, 38, 39,
            40, 50, 51, 52, 53, 45, 57, 58, 59, 49, 61, 71, 63, 73, 65, 55, 56, 57, 67, 68, 60, 72, 62, 74,
            64, 74, 75, 78, 68, 69,
        ];
        let maze_array_1024 = vec![
            -1, 0, 1, 13, 14, 4, 16, 8, 18, 19, 0, 1, 11, 12, 24, 5, 26, 27, 28, 18, 10, 11, 12, 13, 23, 35,
            25, 28, 38, 19, 31, 41, 22, 23, 24, 36, 46, 36, 37, 49, 30, 42, 32, 42, 34, 44, 56, 57, 38, 48,
            51, 41, 42, 52, 53, 65, 55, 56, 57, 58, 50, 62, 52, 62, 54, 66, 76, 66, 78, 79, 60, 61, 71, 72,
            73, 74, 75, 76, 77, 78,
        ];*/
 /*        let maze_array_2048 = vec![
            -1, 0, 3, 13, 14, 4, 7, 17, 18, 19, 11, 1, 22, 12, 13, 14, 15, 18, 28, 29, 10, 20, 21, 22, 34, 
            26, 16, 26, 38, 28, 20, 21, 31, 32, 44, 25, 37, 27, 37, 38, 50, 51, 32, 42, 43, 46, 36, 37, 58, 
            59, 51, 52, 42, 63, 53, 56, 46, 47, 59, 69, 61, 62, 52, 73, 54, 64, 65, 66, 67, 79, 60, 72, 62, 
            72, 75, 65, 66, 78, 68, 78 
        ];  */


        let path_to_test_map = "./data/map_10_8_42.txt";
        let test_map = load_map(path_to_test_map.to_string());
        println!("{}", test_map.edges_matrix);
        let test_maze = Maze {
            width: 10,
            height: 8,
            predecessor: prim_naive_function(&test_map).0,
            cost: prim_naive_function(&test_map).1,
        };
        println!("{}\n", test_maze);
        println!("{}\n", test_maze.cost);

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
