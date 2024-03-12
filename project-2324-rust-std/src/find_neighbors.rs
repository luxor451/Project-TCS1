use crate::core::neighors::*;
use crate::Map;

/// Retrun a Neighbors Struct that represent the neighbors of the node named node_number in map
pub fn find_neighbors(map: &Map, node_nubmer: usize) -> Neighbors {
    // Delinearize node_number
    let x = node_nubmer % map.width;
    let y = node_nubmer / map.width;
    return Neighbors::new_node(map, x, y);
}
