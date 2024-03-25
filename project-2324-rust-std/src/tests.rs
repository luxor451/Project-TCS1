//! Test file, contains all the tests for the objects defined in core.rs

#[cfg(test)]
mod test {

    use csv::Writer;
    use std::{cell::RefCell, rc::Rc, vec};

    use crate::{
        core::{
            binary_heap::{Heap, Node},
            maze::*,
            neighors::Neighbors,
        },
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
        assert_eq!(prim_naive_function(&map_76).1, -695);
        // map 1024
        let path_to_test_map_1024: &str = "./data/map_10_8_1024.txt";
        let map_1024: Map = load_map(path_to_test_map_1024.to_string());
        assert_eq!(prim_naive_function(&map_1024).1, -742);
        // map 2048
        let path_to_test_map_2024: &str = "./data/map_10_8_2048.txt";
        let map_2024: Map = load_map(path_to_test_map_2024.to_string());
        assert_eq!(prim_naive_function(&map_2024).1, -666);
    }

    #[test]
    fn test_father_of_node_n() {
        // This is the heap used in the subject
        let nodes = Node {
            value: 5,
            cost: 5,
            left_child: Some(Rc::new(RefCell::new(Node {
                value: 21,
                cost: 21,
                left_child: Some(Rc::new(RefCell::new(Node {
                    value: 25,
                    cost: 25,
                    left_child: Some(Rc::new(RefCell::new(Node {
                        value: 80,
                        cost: 80,
                        left_child: None,
                        right_child: None,
                    }))),
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(Node {
                    value: 99,
                    cost: 99,
                    left_child: None,
                    right_child: None,
                }))),
            }))),
            right_child: Some(Rc::new(RefCell::new(Node {
                value: 45,
                cost: 45,

                left_child: Some(Rc::new(RefCell::new(Node {
                    value: 75,
                    cost: 75,

                    left_child: None,
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(Node {
                    value: 51,
                    cost: 51,

                    left_child: None,
                    right_child: None,
                }))),
            }))),
        };
        let heap = Heap {
            size: 8,
            root: Some(Rc::new(RefCell::new(nodes))),
            pos: Vec::new(),
        };
        assert_eq!(heap.path_to_father_of_node(1), None);
        assert_eq!(
            heap.path_to_father_of_node(2)
                .unwrap()
                .last()
                .unwrap()
                .borrow()
                .value,
            5
        );
        assert_eq!(
            heap.path_to_father_of_node(3)
                .unwrap()
                .last()
                .unwrap()
                .borrow()
                .value,
            5
        );
        assert_eq!(
            heap.path_to_father_of_node(6)
                .unwrap()
                .last()
                .unwrap()
                .borrow()
                .value,
            45
        );
        assert_eq!(
            heap.path_to_father_of_node(8)
                .unwrap()
                .last()
                .unwrap()
                .borrow()
                .value,
            25
        );
        assert_eq!(
            heap.path_to_father_of_node(5)
                .unwrap()
                .last()
                .unwrap()
                .borrow()
                .value,
            21
        );
    }
    #[test]
    fn test_heap_correctness() {
        let number_of_nodes = 50;
        let mut new_heap: Heap<i32> = Heap::default();
        for i in 0..number_of_nodes {
            let _n = new_heap.insert(Rc::new(RefCell::new(Node {
                value: i,
                cost: i as i64,
                left_child: None,
                right_child: None,
            })));
        }

        let mut test_vec: Vec<i32> = Vec::new();
        // println!("Heap before extraction : \n{:?}", new_heap);
        let min = new_heap.extract_min();
        test_vec.push(min.0.unwrap());
        for _i in 1..number_of_nodes {
            let min = new_heap.extract_min();
            test_vec.push(min.0.unwrap());
        }

        // println!("Heap after extraction : \n{:?}", new_heap);
        assert!(test_vec == (0..number_of_nodes).collect::<Vec<i32>>());
    }

    use std::error::Error;
    #[test]
    // too long to run (around 3 minutes on my machine (Acer Swift 3 2021 with an Ryzen 5 4500H running on Manjaro))
    #[ignore]
    fn test_heap_complexity() -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path("./heap_correctness.csv")?;
        for x in 1..20 {
            let mut new_heap: Heap<i32> = Heap::default();
            let number_of_nodes = 20 * (2 as i32).pow(x);
            let insert_start = std::time::Instant::now();
            for i in 0..number_of_nodes {
                let _newnode = new_heap.insert(Rc::new(RefCell::new(Node {
                    value: i,
                    cost: i as i64,
                    left_child: None,
                    right_child: None,
                })));
            }

            let insert_duration = insert_start.elapsed();

            let extract_start = std::time::Instant::now();
            for _i in 0..number_of_nodes {
                let _min = new_heap.extract_min();
            }
            let time_to_extract = extract_start.elapsed();
            // println!("{} nodes inserted in {} ms, path to father of last node extracted in {} ms", number_of_nodes, insert_duration.as_micros(), time_to_extract.as_micros());

            wtr.write_record(&[
                number_of_nodes.to_string(),
                insert_duration.as_millis().to_string(),
                time_to_extract.as_millis().to_string(),
            ])?
        }
        wtr.flush()?;
        Ok(())
    }
    #[test]
    fn test_change_cost() {
        let number_of_nodes = 20;
        let mut new_heap: Heap<i32> = Heap::default();

        for i in (0..number_of_nodes).filter(|&x| x != 15) {
            let _n = new_heap.insert(Rc::new(RefCell::new(Node {
                value: i,
                cost: i as i64,
                left_child: None,
                right_child: None,
            })));
        }
        let the_15_th_node = Rc::new(RefCell::new(Node {
            value: 15,
            cost: 15,
            left_child: None,
            right_child: None,
        }));
        let n = new_heap.insert(the_15_th_node.clone());
        new_heap.change_cost(the_15_th_node, n, -100);
        assert_eq!(new_heap.extract_min().0.unwrap(), 15);

      
    }
}
