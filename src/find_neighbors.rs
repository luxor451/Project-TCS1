//! Only contains the find_neighbors functions even thought most of the algorithm is in Neighbors::new_node

use crate::{map_load::map_load::Map, neighbors::neighbors::Neighbors};

/// Return a Neighbors Struct that represent the neighbors of the node named node_number in map
///
/// # Example
///
/// ```
/// // Create a new Map
/// let map = Map {
///     // Fill in the fields of the Map struct...
/// };
///
/// // Get the neighbors of the node at position 1
/// let neighbors = find_neighbors(&map, 1);
///
/// // Now, neighbors is a Neighbors struct that represents the neighbors of the node at position 1.
/// ```
pub fn find_neighbors(map: &Map, node_nubmer: usize) -> Neighbors {
    // Delinearize node_number
    let x: usize = node_nubmer % map.width;
    let y: usize = node_nubmer / map.width;
    return Neighbors::new_node(map, x, y);
}
