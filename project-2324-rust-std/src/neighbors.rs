//! Contains the implementation and tests for neighbors.

/// Traits for neighbors are : new_node
pub mod neighbors {

    use crate::map_load::map_load::Map;
    use std::fmt;

    /// Represents the neighbors of a node in a map.
    ///
    /// # Example
    /// ```
    /// // Create a new Neighbors struct
    /// let neighbors = Neighbors {
    ///    nb: 2,
    ///    neighbors_names: [2, 1, -1, -1],
    ///    edges_cost: [0, -14, 0, 0],
    /// };
    /// ```
    /// The neighbors_names array contains the name of the neighbors of the node in the map in the following order : down, right, up, left
    /// `neighbors_name[i]` means that the corresponding neighbor does not exist (i.e. is a wall if `neighbors_name[i] == -1` and,
    /// in this case, the corresponding edges cost is an arbitrary value that should never be read.
    pub struct Neighbors {
        pub nb: usize,
        pub neighbors_names: [i64; 4],
        pub edges_cost: [i64; 4],
    }

    impl Neighbors {
        /// Return the neighbors of the node in position (x, y) in map
        ///
        /// # Example
        /// ```
        /// // Take the map map_2_3_42 given in the data
        /// let path_to_map1: &str = "./data/map_2_3_42.txt";
        /// let map_1 = load_map(path_to_map1.to_string());
        ///
        /// // This is how the neighbors of the node in position (0, 0) in map_2_3_42.txt look like
        /// let neighbors_0: Neighbors = Neighbors {
        ///     nb: 2,
        ///     neighbors_names: [2, 1, -1, -1],
        ///     edges_cost: [0, -14, 0, 0],
        ///  };
        /// assert_eq!(neighbors_0, find_neighbors(&map_1, 0));
        ///
        /// ```
        pub fn new_node(map: &Map, x: usize, y: usize) -> Self {
            let mut nb: usize = 0;
            let mut neighbors_names: [i64; 4] = [-1, -1, -1, -1];
            let mut edges_cost: [i64; 4] = [0, 0, 0, 0];
            let curent_node: i64 = (map.width * y + x) as i64;
            // Iterate throught each case and if the neighbors exist add it to the arrays;
            if y < map.height - 1 {
                nb = nb + 1;
                neighbors_names[0] = curent_node + map.width as i64;
                edges_cost[0] = map.edges_matrix.get(x, y).down;
            }
            if x < map.width - 1 {
                nb = nb + 1;
                neighbors_names[1] = curent_node + 1;
                edges_cost[1] = map.edges_matrix.get(x, y).right;
            }

            if y > 0 {
                nb = nb + 1;
                neighbors_names[2] = curent_node - map.width as i64;
                edges_cost[2] = map.edges_matrix.get(x, y - 1).down;
            }

            if x > 0 {
                nb = nb + 1;
                neighbors_names[3] = curent_node - 1;
                edges_cost[3] = map.edges_matrix.get(x - 1, y).right;
            }

            return Neighbors {
                nb,
                neighbors_names,
                edges_cost,
            };
        }
    }

    /// Print all the value in Neighbors for each neighbors that exist i.e. != -1
    impl std::fmt::Display for Neighbors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;
            for i in 0..4 {
                if self.neighbors_names[i] != -1 {
                    if i != 0 {
                        write!(f, ",\n ")?;
                    }
                    write!(f, "({} : {})", self.neighbors_names[i], self.edges_cost[i])?;
                }
            }
            write!(f, "]")
        }
    }

    /// Print ALL the value in Neighbors
    impl std::fmt::Debug for Neighbors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;
            for i in 0..4 {
                if i != 0 {
                    write!(f, ",\n ")?;
                }
                write!(f, "({} : {})", self.neighbors_names[i], self.edges_cost[i])?;
            }
            write!(f, "]")
        }
    }
    /// Define equality for Neighbors : two neighbors are equal iff they have the same neighbors with the same edges cost
    impl PartialEq for Neighbors {
        fn eq(&self, other: &Self) -> bool {
            let mut res: bool = true;
            res &= self.nb == other.nb;
            for i in 0..4 {
                if (self.neighbors_names[i] != -1) && (other.neighbors_names[i] != -1) {
                    res &= (self.neighbors_names[i] == other.neighbors_names[i])
                        && (self.edges_cost[i] == other.edges_cost[i]);
                }
            }
            return res;
        }
    }
    /// Define how to iterate throught Neighbors, every iterable is a tuple (i64, i64) with the name of the neighbor and the associated edge cost
    impl IntoIterator for Neighbors {
        type Item = (i64, i64);
        type IntoIter = std::vec::IntoIter<Self::Item>;
        fn into_iter(self) -> Self::IntoIter {
            let mut vec: Vec<(i64, i64)> = Vec::new();
            for i in 0..4 {
                vec.push((self.neighbors_names[i].clone(), self.edges_cost[i]));
            }
            return vec.into_iter();
        }
    }
}

#[cfg(test)]

mod test_neighbors {

    use crate::{
        find_neighbors::find_neighbors, map_loader::load_map, neighbors::neighbors::Neighbors,
    };

    #[test]
    fn test_find_neighbors() {
        let path_to_map1: &str = "./data/map_2_3_42.txt";
        let map_1 = load_map(path_to_map1.to_string());
        // All the Neighbors for each node in the graph represented in map_2_3_42.txt
        let neighbors_0: Neighbors = Neighbors {
            nb: 2,
            neighbors_names: [2, 1, -1, -1],
            edges_cost: [0, -14, 0, 0],
        };
        let neighbors_1: Neighbors = Neighbors {
            nb: 2,
            neighbors_names: [3, -1, -1, 0],
            edges_cost: [-19, 0, 0, -14],
        };
        let neighbors_2: Neighbors = Neighbors {
            nb: 3,
            neighbors_names: [4, 3, 0, -1],
            edges_cost: [18, -8, 0, 0],
        };
        let neighbors_3: Neighbors = Neighbors {
            nb: 3,
            neighbors_names: [5, -1, 1, 2],
            edges_cost: [0, 0, -19, -8],
        };
        let neighbors_4: Neighbors = Neighbors {
            nb: 2,
            neighbors_names: [-1, 5, 2, -1],
            edges_cost: [0, -5, 18, 0],
        };
        let neighbors_5: Neighbors = Neighbors {
            nb: 2,
            neighbors_names: [-1, -1, 3, 4],
            edges_cost: [0, 0, 0, -5],
        };

        print!("find_neighbors test :\n");
        assert_eq!(neighbors_0, find_neighbors(&map_1, 0));
        assert_eq!(neighbors_1, find_neighbors(&map_1, 1));
        assert_eq!(neighbors_2, find_neighbors(&map_1, 2));
        assert_eq!(neighbors_3, find_neighbors(&map_1, 3));
        assert_eq!(neighbors_4, find_neighbors(&map_1, 4));
        assert_eq!(neighbors_5, find_neighbors(&map_1, 5));
        println!("find_neighbors test passed");
    }
}
