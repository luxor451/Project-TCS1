//! Contains the implementation of the load_map function

use core::*;
use scanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::map_load::map_load::Edges;
use crate::map_load::map_load::Map;
use crate::map_load::map_load::Matrix;

/// Return a map struct that represent the map given at filename
///
///  # Exemple
/// ```
/// // Load a map from a file
/// let map = load_map("./data/map_10_8_42.txt".to_string());
/// ```
pub fn load_map(filename: String) -> Map {
    // Read file
    let file: File = File::open(&filename).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line: String = String::new();
    reader.read_line(&mut line).unwrap();

    // Read and print dimensions in the first line
    let mut width: usize = 0;
    let mut height: usize = 0;
    sscanf!(&line, "{} {}", width, height).unwrap();

    // Initialze the edges matrix
    let mut vector_temp: Vec<Edges> = Vec::with_capacity(width * height);
    for _i in 0..width * height {
        vector_temp.push(Edges { down: 0, right: 0 });
    }
    let mut edges_matrix: Matrix<Edges> = Matrix::new(vector_temp, width, height);

    // Actually do the reading
    let mut below: i64 = 0;
    let mut right: i64 = 0;
    for x in 0..height {
        for y in 0..width {
            line = String::new();
            reader.read_line(&mut line).unwrap();
            sscanf!(&line, "{i64} {i64}", below, right).unwrap();
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
