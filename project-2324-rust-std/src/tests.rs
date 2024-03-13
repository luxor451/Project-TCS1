//! Test file, contains all the tests for the objects defined in core.rs

#[cfg(test)]
mod test {
    use crate::{
        core::{maze::*, neighors::Neighbors},
        find_neighbors::find_neighbors,
        map_loader::load_map,
        prim_naive::prim_naive_function,
        Map,
    };
    #[test]
    fn test_find_neighbors() {
        let path_to_map1: &str = "./data/map_2_3_42.txt";
        let map_1 = load_map(path_to_map1.to_string());
        // All the Neighbors for each node in the graph represented in map_2_3_42.txt
        let neighbors_0: Neighbors = Neighbors {
            nb: 2,
            neighbors_names: [2, 1, -1, -1],
            edges_cost: [0, -14, 0, 0],
        };
        let neighbors_1: Neighbors = Neighbors {
            nb: 2,
            neighbors_names: [3, -1, -1, 0],
            edges_cost: [-19, 0, 0, -14],
        };
        let neighbors_2: Neighbors = Neighbors {
            nb: 3,
            neighbors_names: [4, 3, 0, -1],
            edges_cost: [18, -8, 0, 0],
        };
        let neighbors_3: Neighbors = Neighbors {
            nb: 3,
            neighbors_names: [5, -1, 1, 2],
            edges_cost: [0, 0, -19, -8],
        };
        let neighbors_4: Neighbors = Neighbors {
            nb: 2,
            neighbors_names: [-1, 5, 2, -1],
            edges_cost: [0, -5, 18, 0],
        };
        let neighbors_5: Neighbors = Neighbors {
            nb: 2,
            neighbors_names: [-1, -1, 3, 4],
            edges_cost: [0, 0, 0, -5],
        };

        print!("find_neighbors test :\n");
        assert_eq!(neighbors_0, find_neighbors(&map_1, 0));
        assert_eq!(neighbors_1, find_neighbors(&map_1, 1));
        assert_eq!(neighbors_2, find_neighbors(&map_1, 2));
        assert_eq!(neighbors_3, find_neighbors(&map_1, 3));
        assert_eq!(neighbors_4, find_neighbors(&map_1, 4));
        assert_eq!(neighbors_5, find_neighbors(&map_1, 5));
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
        let maze_2048: Maze = Maze {
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
        let maze_2048: Maze = Maze {
            width: 10,
            height: 8,
            predecessor: maze_array_2048,
            cost: -666,
        };
        let _ = Maze::write_maze_in_pbm(&maze_2048);
    }
    #[test]
    fn test_prim_naif() {
        // Test for each map provided in tests_map
        // Test for the cost and not the actual tree because two different trees can be minimal
        println!("Prim's algotithm naive test : \n");
        // map 42
        let path_to_test_map_42: &str = "./data/map_10_8_42.txt";
        let map_42: Map = load_map(path_to_test_map_42.to_string());
        // map 76
        assert_eq!(prim_naive_function(&map_42).1, -762);
        let path_to_test_map_76: &str = "./data/map_10_8_76.txt";
        let map_76: Map = load_map(path_to_test_map_76.to_string());
        assert_eq!(prim_naive_function(&map_76).1, -695
    );
        // map 1024
        let path_to_test_map_1024: &str = "./data/map_10_8_1024.txt";
        let map_1024: Map = load_map(path_to_test_map_1024.to_string());
        assert_eq!(prim_naive_function(&map_1024).1, -742);
        // map 2048
        let path_to_test_map_2024: &str = "./data/map_10_8_2048.txt";
        let map_2024: Map = load_map(path_to_test_map_2024.to_string());
        assert_eq!(prim_naive_function(&map_2024).1, -666);
    }
}
