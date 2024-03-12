//! Contains the implementation of the load_map function

use crate::core::map_load::*;
use core::*;
use scanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Return a map struct that represent the map given at filename
pub fn load_map(filename: String) -> Map {
    //read file
    let file: File = File::open(&filename).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line: String = String::new();
    reader.read_line(&mut line).unwrap();

    // read and print dimensions in the first line
    let mut width: usize = 0;
    let mut height: usize = 0;
    sscanf!(&line, "{} {}", width, height).unwrap();

    //initialze the edges matrix
    let mut vector_temp: Vec<Edges> = Vec::with_capacity(width * height);
    for _i in 0..width * height {
        vector_temp.push(Edges { down: 0, right: 0 });
    }
    let mut edges_matrix: Matrix<Edges> = Matrix::new(vector_temp, width, height);

    //actually read the map
    let mut below: i32 = 0;
    let mut right: i32 = 0;
    for x in 0..height {
        for y in 0..width {
            line = String::new();
            reader.read_line(&mut line).unwrap();
            sscanf!(&line, "{i32} {i32}", below, right).unwrap();
            *edges_matrix.get_mut(x, y) = Edges {
                down: below,
                right: right,
            };
        }
    }
    let map: Map = Map {
        width,
        height,
        edges_matrix,
    };
    return map;
}
