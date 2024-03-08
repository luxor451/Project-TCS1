use std::{cell, fmt::*};
struct Cell<T> {
    hd: T,
    tl: List<T>,
}
pub struct List<T>(Option<Box<Cell<T>>>);

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        let mut cur =  self;
        while let Some(cell) = &cur.0{
            write!(f, "{}, ", cell.hd);
            cur = &cell.tl
        }
        write!(f, "]")?;
        return Ok(());
    }
}
impl<T> List<T> {
    // utilitary functions go here

    // functions from the signature
    pub fn nil() -> List<T> {
        List(None)
    }
    pub fn cons(t: T, q: List<T>) -> List<T> {
        List(Some(Box::new(Cell {hd: t, tl :q})))
    }
    pub fn size(&self) -> usize {
        let mut res = 0;
        while self != List(None) {
            
        }
        return res;
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
