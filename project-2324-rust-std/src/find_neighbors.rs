//! Only contains the find_neighbors functions even thought most of the algorithm is in core::neighbors::Neighbors::new_node

use crate::core::neighors::*;
use crate::Map;

/// Retrun a Neighbors Struct that represent the neighbors of the node named node_number in map
pub fn find_neighbors(map: &Map, node_nubmer: usize) -> Neighbors {
    // Delinearize node_number
    let x: usize = node_nubmer % map.width;
    let y: usize = node_nubmer / map.width;
    return Neighbors::new_node(map, x, y);
}
