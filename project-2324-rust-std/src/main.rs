use projet::map_load::*;
pub mod map_loader;

use crate::map_loader::*;


fn main(){
    //Matrix test
    let mut mv = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    *mv.get_mut(1, 2) = 10;
    println!("Display: {}", mv);
    println!("Debug: {:?}", mv);
    //load_map test
    let filename: &str = "./data/map_2_3_42.txt";
    let test_map = load_map(filename.to_string());
    println!("Display: {}", test_map.edges_matrix);
    
}