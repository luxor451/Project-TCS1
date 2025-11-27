//! Implement Prim's algorithm naively, the shearch for the vertex that minimize cheapest is linear

use std::collections::HashSet;

use crate::{
    find_neighbors::find_neighbors, map_load::map_load::Map, neighbors::neighbors::Neighbors,
};

/// Naive implementation of prim's algorithm as given in the subject
/// Take a map and return the predecessor array and the cost of the minimum spanning tree as tuple
/// It make use of linear schearch to find the node that minimize cheapest, this give it a complexity of O(n^2)
///
///
/// ```
/// let mut arg_min: i64 = *capitalize_n.iter().next().unwrap();
/// let mut min: i64 = cheapest[arg_min as usize];
///
/// for vertex in capitalize_n.iter() {
///    if cheapest[*vertex as usize] < min {
///        arg_min = *vertex;
///        min = cheapest[*vertex as usize];
///    }
/// }
/// v = arg_min;
/// ```
/// If you want you can use this function to get the minimum you can replace the line 40 by the code above to really
/// show that's it's linear shearch
pub fn prim_naive_function(map: &Map) -> (Vec<i64>, i64) {
    // Initialize all the variables
    let maze_size: usize = map.width * map.height;
    let mut cheapest: Vec<i64> = vec![i64::MAX; maze_size];
    let mut predecessor: Vec<i64> = vec![-1; maze_size];
    let mut capitalize_n: HashSet<i64> = (0..maze_size as i64).collect();
    // We chose the node 0;
    let mut v: i64 = 0;
    cheapest[v as usize] = 0;

    while !capitalize_n.is_empty() {
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

#[cfg(test)]
mod test_prim_naif {
    use crate::{map_load::map_load::Map, map_loader::load_map, prim_naive::prim_naive_function};

    #[test]
    fn test_prim_n() {
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
}
