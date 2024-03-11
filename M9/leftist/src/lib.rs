use std::fmt::{Display, Formatter, Result};

pub trait PriorityQueue<K: Ord, V> {
    fn empty() -> Self;
    fn insert(&mut self, key: K, value: V);
    fn merge(&mut self, other: &mut Self);
    fn get_min(&self) -> Option<(&K, &V)>;
    fn extract_min(&mut self) -> Option<(K, V)>;
}
// get rid of the dummy fields
// and provide a correct implementation for the type LBTree<K:Ord, V>
pub struct LBTree<K: Ord, V> {
    _dummy1: K,
    _dummy2: V,
}

impl<K: Ord + Display, V: Display> Display for LBTree<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        panic!("not implemented !")
    }
}

impl<K: Ord, V> LBTree<K, V> {
    // here write auxiliary functions
}
impl<K: Ord, V> PriorityQueue<K, V> for LBTree<K, V> {
    fn empty() -> Self {
        panic!("not implemented !")
    }
    fn merge(&mut self, other: &mut Self) {
        panic!("not implemented !")
    }
    fn insert(&mut self, key: K, value: V) {
        panic!("not implemented !")
    }
    fn get_min(&self) -> Option<(&K, &V)> {
        panic!("not implemented !")
    }
    fn extract_min(&mut self) -> Option<(K, V)> {
        panic!("not implemented !")
    }
}
