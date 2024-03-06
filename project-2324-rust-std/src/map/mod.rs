
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use scanf::sscanf;
use crate::map_load::*;

pub fn load_map(filename : String) -> Map{
    assert_eq!(filename.len(), 2, "You must provide a file name");

    let file = File::open(&filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    // read and print dimensions in the first line
    let mut width: usize = 0;
    let mut height: usize = 0;

    sscanf!(&line, "{} {}", width, height).unwrap();

    let mut edges_matrix = Matrix::new(vec![], width, height);

    for x in 0..width {
        for y in 0..height{
            let mut line = String::new();
            let mut below: i32 = 0;
            let mut right: i32 = 0;
            reader.read_line(&mut line).unwrap();
            sscanf!(&line, "{i32} {i32}", below, right).unwrap();

            *edges_matrix.get_mut(x, y) = Edges {down : below, right :right};
        }
    }

    let map : Map = Map {width, height, edges_matrix};
    return map
}

