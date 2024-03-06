use std::fmt::*;
struct Cell<T> {
    hd: T,
    tl: List<T>,
}
pub struct List<T>(Option<Box<Cell<T>>>);

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        panic!("not yet implemented")
    }
}
impl<T> List<T> {
    // utilitary functions go here

    // functions from the signature
    pub fn nil() -> List<T> {
        panic!("not yet implemented")
    }
    pub fn cons(t: T, q: List<T>) -> List<T> {
        panic!("not yet implemented")
    }
    pub fn size(&self) -> usize {
        panic!("not yet implemented")
    }
    pub fn get_element(&self, pos: usize) -> &T {
        panic!("not yet implemented")
    }
    pub fn insert_element(&mut self, pos: usize, t: T) {
        panic!("not yet implemented")
    }
    pub fn remove_element(&mut self, pos: usize) -> T {
        panic!("not yet implemented")
    }
}
