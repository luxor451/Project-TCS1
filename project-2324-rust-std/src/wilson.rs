//! Implementation of wilson's algorithm
//!
//! Documentation that i used : <https://www.cs.cmu.edu/~15859n/RelatedWork/RandomTrees-Wilson.pdf>
use crate::{
    find_neighbors::find_neighbors, map_load::map_load::Map, maze::maze::Maze,
    neighbors::neighbors::Neighbors,
};
use rand::seq::SliceRandom;

use image::{ImageBuffer, Rgb};
extern crate image;
use std::{fs, process::Command, vec};

/// Create a png file of the current state of the algorithm and save it in the video/images folder as output-{i}.png
fn write_in_png(next: Vec<i64>, width: usize, height: usize, i: usize) {
    let maze = Maze {
        width: width,
        height: height,
        predecessor: next,
        cost: 0,
    };
    let _ = maze.write_maze_in_pbm(&"video/buffer.txt".to_string());

    // a default (black) image containing Rgb values
    let filename = "video/buffer.txt";
    let content = fs::read_to_string(filename).unwrap();
    let content: Vec<&str> = content.split_whitespace().collect();

    let w = content[1].parse::<u32>().unwrap();
    let h = content[2].parse::<u32>().unwrap();
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w * 5, h * 5);

    for x in 0..w {
        for y in 0..h {
            let index = (y * w + x) as usize;
            let value = content[index + 3].parse::<u8>().unwrap();
            for i in 0..5 {
                for j in 0..5 {
                    if value == 1 {
                        image.get_pixel_mut(x * 5 + i, y * 5 + j).0 = [0, 0, 0];
                    } else {
                        image.get_pixel_mut(x * 5 + i, y * 5 + j).0 = [255, 255, 255];
                    }
                }
            }
        }
    }

    image.get_pixel_mut(5, 5).0 = [255, 255, 255];

    image.save(format!("video/images/output-{i}.png")).unwrap();
}

/// Take a `node` in a `map` and return a random neighbor of this node
fn random_succesor(map: &Map, node: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let neighbors: Neighbors = find_neighbors(map, node as usize);
    let valid_neighbors: Vec<_> = neighbors
        .into_iter()
        .filter(|(node, _)| *node != -1)
        .collect();
    let random_neighbors = valid_neighbors.choose(&mut rng).unwrap();
    return random_neighbors.0;
}
/// Implement wilson algorithm, return an array Next representing a spanning tree choosen
/// uniformly amongst all posible spanning tree
///
/// Next of i is the next node of i in the spanning tree
pub fn wilson_algorithm_with_root(map: &Map, root: i64) -> Vec<i64> {
    // This is almost exactly the implementation in wilson's paper
    let maze_size: usize = map.width * map.height;
    let mut next: Vec<i64> = vec![-1; maze_size];
    let mut intree = vec![false; maze_size];
    next[root as usize] = -1;
    intree[root as usize] = true;
    let mut u;
    for i in 0..maze_size {
        if !intree[i] {
            u = i;
            // Perform a random walk to the next node
            while !intree[u] {
                next[u] = random_succesor(map, u as i64);
                u = next[u] as usize;
            }
            u = i;
            // Erase all cycle
            while !intree[u] {
                intree[u] = true;
                u = next[u] as usize;
            }
        }
    }

    // The algorithme in the paper return a array of the following node, the convention in this project is to use
    // predecessor arrays but this still work with the print function defined in maze.rs because the logic is the same
    return next;
}

/// Execute wilson's algorithm and generate a 30 seconds video showing its execution
pub fn wilson_algorithm_with_root_with_video(map: &Map, root: i64) -> Vec<i64> {
    // Create the necessary folders and remove the old video if it exist
    Command::new("bash")
        .arg("-c")
        .arg("rm maze_generation.mp4")
        .output()
        .expect("failed to remove old video file");
    Command::new("bash")
        .arg("-c")
        .arg("mkdir video")
        .output()
        .expect("failed to create video folder");
    Command::new("bash")
        .arg("-c")
        .arg("mkdir video/images")
        .output()
        .expect("failed to create buffer file");
    // The algorithm itself
    let maze_size: usize = map.width * map.height;
    let mut next: Vec<i64> = vec![-1; maze_size];
    let mut intree = vec![false; maze_size];
    next[root as usize] = -1;
    intree[root as usize] = true;
    let mut u;
    let mut img_counter = 0;
    for i in 0..maze_size {
        if !intree[i] {
            u = i;
            write_in_png(next.clone(), map.width, map.height, img_counter);
            img_counter += 1;
            while !intree[u] {
                next[u] = random_succesor(map, u as i64);
                u = next[u] as usize;
            }
            u = i;
            while !intree[u] {
                intree[u] = true;
                u = next[u] as usize;
            }
        }
    }
    // Create the new video
    Command::new("bash")
        .arg("-c")
        .arg(format!("ffmpeg -framerate {} -i video/images/output-%d.png -c:v libx264 -r 30 maze_generation.mp4", img_counter/30)) // make it last 30 seconds
        .output()
        .expect("failed to execute ffmpeg command");
    // Clean the images used to create the video
    Command::new("bash")
        .arg("-c")
        .arg("rm -r video")
        .output()
        .expect("failed to remove video folder");
    return next;
}

#[cfg(test)]
mod test_prim_bh {

    use crate::{
        map_load::map_load::Map,
        map_loader::load_map,
        maze::maze::Maze,
        wilson::{wilson_algorithm_with_root, wilson_algorithm_with_root_with_video},
    };

    #[test]
    fn test_wilson() {
        // The only test is just to visualy confirm it is a maze
        let path_to_test_map_42: &str = "./data/map_10_8_42.txt";
        let map_42: Map = load_map(path_to_test_map_42.to_string());
        let pred = wilson_algorithm_with_root(&map_42, 0);
        println!("{:?}", pred);
        let maze_42: Maze = Maze {
            width: 10,
            height: 8,
            predecessor: pred,
            cost: 0,
        };
        println!("\n{} \n ^^ Maze generated with wilson", maze_42);
    }
    #[test]
    fn test_wilson_video() {
        let path_to_test_map_42: &str = "./data/map_10_8_42.txt";
        let map_42: Map = load_map(path_to_test_map_42.to_string());
        wilson_algorithm_with_root_with_video(&map_42, 10);
    }
}
