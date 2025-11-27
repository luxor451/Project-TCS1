//! Implementation of Prim's algorithm using binary heaps.
//! 
//! For the prim_bh function, to change the cost of a node I choose to insert a new node and keep an Vec
//! giving if we have already visited the node or not

use std::{cell::RefCell, rc::Rc};

use crate::{
    binary_heap::binary_heap::{Heap, Node},
    find_neighbors::find_neighbors,
    map_load::map_load::Map,
    neighbors::neighbors::Neighbors,
};

/// Implementation of Prim's algorithm using binary heaps, take a map and return the predecessor array and the cost of the minimum spanning tree
/// It make use of a binary heap to keep track of the nodes with the lowest cost, this give it a complexity of O(nlog(n))
pub fn prim_bh_funtion(map: &Map) -> (Vec<i64>, i64) {
    // Initialize all the variables
    let maze_size: usize = map.width * map.height;
    let mut predecessor: Vec<i64> = vec![-1; maze_size];
    let mut heap = Heap::default();
    // Like said in the subject, we use a vec to keep track of the nodes that have been seen
    let mut as_been_seen = vec![false; maze_size];
    let mut cheapest = vec![i64::MAX; maze_size];
    // Insert all the nodes in the heap
    for i in 0..maze_size {
        let node_to_insert = Rc::new(RefCell::new(Node {
            cost: i64::MAX as i64,
            value: i as i64,
            left_child: None,
            right_child: None,
        }));
        heap.insert(node_to_insert.clone());
    }
    // We chose the node 0;
    let mut v: i64 = 0;
    heap.insert(Rc::new(RefCell::new(Node {
        cost: 0,
        value: v,
        left_child: None,
        right_child: None,
    })));
    cheapest[v as usize] = 0;
    while !heap.is_empty() {
        // Get the node with the minimun cost in the heap
        v = heap.extract_min().unwrap();
        // If the node has already been seen, we skip to the next iteration
        if !as_been_seen[v as usize] {
            as_been_seen[v as usize] = true;
            let neighbors: Neighbors = find_neighbors(map, v as usize);
            for (node, edge_cost) in neighbors {
                if node != -1 {
                    if !as_been_seen[node as usize] && edge_cost < cheapest[node as usize] {
                        // Even if the node is already in the heap, we need to update it's cost but like said in the subject
                        // Since the "new" node as a lower cost it will be seen first
                        heap.insert(Rc::new(RefCell::new(Node {
                            cost: edge_cost as i64,
                            value: node,
                            left_child: None,
                            right_child: None,
                        })));
                        cheapest[node as usize] = edge_cost;
                        predecessor[node as usize] = v;
                    }
                }
            }
        }
    }
    return (predecessor, cheapest.iter().sum());
}

#[cfg(test)]
mod test_prim_bh {
    // These are the same tests as the ones in prim_naive.rs
    use crate::{map_load::map_load::Map, map_loader::load_map, prim_bh::prim_bh_funtion};

    #[test]
    fn test_prim_n() {
        // Test for each map provided in tests_map
        // Test for the cost and not the actual tree because two different trees can be minimal
        println!("Prim's algotithm naive test : \n");
        // map 42
        let path_to_test_map_42: &str = "./data/map_10_8_42.txt";
        let map_42: Map = load_map(path_to_test_map_42.to_string());
        // map 76
        assert_eq!(prim_bh_funtion(&map_42).1, -762);
        let path_to_test_map_76: &str = "./data/map_10_8_76.txt";
        let map_76: Map = load_map(path_to_test_map_76.to_string());
        assert_eq!(prim_bh_funtion(&map_76).1, -695);
        // map 1024
        let path_to_test_map_1024: &str = "./data/map_10_8_1024.txt";
        let map_1024: Map = load_map(path_to_test_map_1024.to_string());
        assert_eq!(prim_bh_funtion(&map_1024).1, -742);
        // map 2048
        let path_to_test_map_2024: &str = "./data/map_10_8_2048.txt";
        let map_2024: Map = load_map(path_to_test_map_2024.to_string());
        assert_eq!(prim_bh_funtion(&map_2024).1, -666);
    }
}

