use crate::{core::neighors::Neighbors, find_neighbors::find_neighbors, Map};
use std::collections::HashSet;

/// Naive implementation of prim's algorithm as given in the subject
/// Use of linear schearch
pub fn prim_naive_function(map: &Map) -> (Vec<i32>, i32) {
    // Initialize all the variables
    let maze_size: usize = map.width * map.height;
    let mut cheapest: Vec<i32> = vec![i32::MAX; maze_size];
    let mut predecessor: Vec<i32> = vec![-1; maze_size];
    let mut capitalize_n: HashSet<i32> = (0..maze_size as i32).collect();
    // We chose the node 0;
    let mut v: i32 = 0;
    cheapest[v as usize] = 0;

    while !capitalize_n.is_empty() {
        // Get the node that minmize cheapest, other implementation in quote below
        v = *capitalize_n
            .iter()
            .min_by_key(|&node| cheapest[*node as usize])
            .unwrap();
        /* let mut arg_min: i32 = *capitalize_n.iter().next().unwrap();
        let mut min: i32 = cheapest[arg_min as usize];

        for vertex in capitalize_n.iter() {
            if cheapest[*vertex as usize] < min {
                arg_min = *vertex;
                min = cheapest[*vertex as usize];
            }
        }
        v = arg_min */
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
