use crate::map_load::*;
use crate::neighors::*;

pub fn find_neighbors(map: &Map, node_nubmer: usize) -> Neighbors {
    let x = node_nubmer % map.width;
    let y = node_nubmer / map.width;
    return Neighbors::new_node(map, x, y);
}
