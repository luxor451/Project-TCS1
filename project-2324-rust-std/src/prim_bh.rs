
use std::{cell::RefCell, rc::Rc};

use crate::{core::{binary_heap::{Heap, Node, NodeCell}, neighors::Neighbors}, find_neighbors::find_neighbors, Map};

pub fn prim_naive_function(map: &Map) -> (Vec<i32>, i32) {
    // Initialize all the variables
    let maze_size: usize = map.width * map.height;
    let mut predecessor: Vec<i32> = vec![-1; maze_size];
    let mut heap  = Heap::default();
    let mut all_nodes: Vec<Rc<NodeCell<i32>>> = Vec::with_capacity(maze_size);
    for i in 0..maze_size {
        let node_to_insert = Rc::new(RefCell::new( Node {
            cost: i64::MAX,
            value: i as i32,
            left_child: None,
            right_child: None,
        }));
        let _n = heap.insert(node_to_insert.clone());
        all_nodes.push(node_to_insert);
    }
    // We chose the node 0;
    let mut v: i32 = 0;
    
    while !heap.is_empty() {
        // Get the node that minmize cheapest, other implementation in quote below
        v = *capitalize_n
            .iter()
            .min_by_key(|&node| cheapest[*node as usize])
            .unwrap();

        capitalize_n.remove(&v);

        let neighbors: Neighbors = find_neighbors(map, v as usize);
        for (node, edge_cost) in neighbors {
            if node != -1 {
                if capitalize_n.contains(&node) && edge_cost < cheapest[node as usize] {
                    cheapest[node as usize] = edge_cost;
                    predecessor[node as usize] = v;
                }
            }
        }
    }
    return (predecessor, cheapest.iter().sum());
}