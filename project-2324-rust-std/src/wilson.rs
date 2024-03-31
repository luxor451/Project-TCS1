//! Implementation of wilson's algorithm
//!
//! Documentation that i used : https://www.cs.cmu.edu/~15859n/RelatedWork/RandomTrees-Wilson.pdf
//!

use crate::{
    find_neighbors::find_neighbors, map_load::map_load::Map, neighbors::neighbors::Neighbors,
};
use rand::seq::SliceRandom;

/// Take a `node` in a `map` and return a random neighbor of this node
fn random_succesor(map: &Map, node: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let neighbors: Neighbors = find_neighbors(map, node as usize);
    let valid_neighbors: Vec<_> = neighbors
        .into_iter()
        .filter(|(node, _)| *node != -1)
        .collect();
    let random_neighbors = valid_neighbors.choose(&mut rng).unwrap();
    return random_neighbors.0;
}
/// Implement wilson algorithm, return an array Next representing a spanning tree choosen
/// uniformly amongst all posible spanning tree
///
/// Next[i] is the next node of i in the spanning tree
pub fn wilson_algorithm_with_root(map: &Map, root: i64) -> Vec<i64> {
    // This is almost exactly the implementation in wilson's paper
    let maze_size: usize = map.width * map.height;
    let mut next: Vec<i64> = vec![-1; maze_size];
    let mut intree = vec![false; maze_size];
    next[root as usize] = -1;
    intree[root as usize] = true;
    let mut u;
    for i in 0..maze_size {
        u = i;
        // Perform a random walk to the next node
        while !intree[u] {
            next[u] = random_succesor(map, u as i64);
            u = next[u] as usize;
        }
        u = i;
        // Erase all cycle
        while !intree[u] {
            intree[u] = true;
            u = next[u] as usize;
        }
    }
    // The algorithme in the paper return a array of the following node, the convention in this project is to use
    // predecessor arrays but this still work with the print function defined in maze.rs because the logic is the same
    return next;
}

#[cfg(test)]
mod test_prim_bh {
    use crate::{
        map_load::map_load::Map, map_loader::load_map, maze::maze::Maze,
        wilson::wilson_algorithm_with_root,
    };

    #[test]
    fn test_prim_n() {
        // The only test is just to visualy confirm it is a maze
        let path_to_test_map_42: &str = "./data/map_10_8_42.txt";
        let map_42: Map = load_map(path_to_test_map_42.to_string());
        let pred = wilson_algorithm_with_root(&map_42, 0);
        println!("{:?}", pred);
        let maze_42: Maze = Maze {
            width: 10,
            height: 8,
            predecessor: pred,
            cost: 0,
        };
        println!("\n{} \n ^^ Maze generated with wilson", maze_42);
    }
}
