pub mod map_load {
    use std::fmt;

    pub struct Edges {
        pub down: i32,
        pub right: i32,
    }

    pub struct Map {
        pub width: usize,
        pub height: usize,
        pub edges_matrix: Matrix<Edges>,
    }

    #[derive(Debug)]
    pub struct Matrix<T> {
        pub vec: Vec<T>,
        pub row: usize,
        pub col: usize,
    }

    impl<T> Matrix<T> {
        pub fn new(vec: Vec<T>, n_col: usize, n_row: usize) -> Self {
            assert!(vec.len() == n_row * n_col);
            Self {
                vec: vec,
                row: n_row,
                col: n_col,
            }
        }

        pub fn row(&self, row: usize) -> &[T] {
            let i = self.col * row;
            &self.vec[i..(i + self.col)]
        }

        pub fn get(&self, x: usize, y: usize) -> &T {
            let i = self.col * y;
            &self.vec[i + x]
        }

        pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
            let i = self.col * x;
            &mut self.vec[i + y]
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Display for Matrix<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut str = String::new();
            for i in 0..self.row {
                if i != 0 {
                    str.push_str(",\n ");
                }
                str.push_str(&format!("{:?}", &self.row(i)));
            }
            write!(f, "[\n{}\n]", str)
        }
    }

    impl std::fmt::Display for Edges {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(down : {} ; right : {})", self.down, self.right)
        }
    }
    impl std::fmt::Debug for Edges {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(down : {} ; right : {})", self.down, self.right)
        }
    }
}

pub mod neighors {
    use crate::core::map_load::*;
    use std::fmt;

    pub struct Neighbors {
        pub nb: usize,
        pub neighbors_names: [i32; 4],
        pub edges_cost: [i32; 4],
    }

    impl Neighbors {
        pub fn empty_node() -> Self {
            Self {
                nb: 0,
                neighbors_names: [0, 0, 0, 0],
                edges_cost: [0, 0, 0, 0],
            }
        }
        pub fn new_node(map: &Map, x: usize, y: usize) -> Self {
            let mut nb: usize = 0;
            let mut neighbors_names: [i32; 4] = [-1, -1, -1, -1];
            let mut edges_cost: [i32; 4] = [0, 0, 0, 0];
            let curent_node: i32 = (map.width * y + x) as i32;
            if x > 0 {
                nb = nb + 1;
                neighbors_names[3] = curent_node - 1;
                edges_cost[3] = map.edges_matrix.get(x - 1, y).right;
            }
            if x < map.width - 1 {
                nb = nb + 1;
                neighbors_names[1] = curent_node + 1;
                edges_cost[1] = map.edges_matrix.get(x, y).right;
            }
            if y > 0 {
                nb = nb + 1;
                neighbors_names[2] = curent_node - map.width as i32;
                edges_cost[2] = map.edges_matrix.get(x, y - 1).down;
            }
            if y < map.height - 1 {
                nb = nb + 1;
                neighbors_names[0] = curent_node + map.width as i32;
                edges_cost[0] = map.edges_matrix.get(x, y).down;
            }

            return Neighbors {
                nb,
                neighbors_names,
                edges_cost,
            };
        }
    }

    impl std::fmt::Display for Neighbors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut str = String::new();
            for i in 0..4 {
                if self.neighbors_names[i] != -1 {
                    if i != 0 {
                        str.push_str(",\n ");
                    }
                    str.push_str(&format!(
                        "({} : {})",
                        self.neighbors_names[i], self.edges_cost[i]
                    ));
                }
            }
            write!(f, "[{}]", str)
        }
    }
    impl std::fmt::Debug for Neighbors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut str = String::new();
            for i in 0..4 {
                if i != 0 {
                    str.push_str(",\n ");
                }
                str.push_str(&format!(
                    "({} : {})",
                    self.neighbors_names[i], self.edges_cost[i]
                ));
            }
            write!(f, "[{}]", str)
        }
    }
    impl PartialEq for Neighbors {
        fn eq(&self, other: &Self) -> bool {
            self.nb == other.nb
                && self.neighbors_names == other.neighbors_names
                && self.edges_cost == other.edges_cost
        }
    }
}

pub mod maze {
    use std::{fmt, fs::File, io::BufWriter};
    use std::io::Write;
    
    pub struct Maze {
        pub width: usize,
        pub height: usize,
        pub predecessor: Vec<i32>,
        pub cost: i32,
    }

    impl Maze {
        pub fn write_maze_in_pbm(&self) -> std::io::Result<()> {
            let file = File::create("print_maze.pbm")?;
            // First row of wall
            let mut writer = BufWriter::new(&file);
            write!(writer, "P1\n{} {}\n", 2 * self.width + 1, 2 * self.height + 1)?;
            write!(writer, "{}", &"1 ".repeat(2 * self.width))?; 
            write!(writer, "1\n")?;


            for y in 0..self.height - 1 {
                write!(writer, "1 ")?;
                for x in 0..self.width - 1 {
                    write!(writer, "0 ")?;
                    if (self.predecessor[(y * self.width) + x + 1] == ((y * self.width) + x) as i32)
                        || (self.predecessor[(y * self.width) + x]
                            == ((y * self.width) + x + 1) as i32)
                    {
                        write!(writer, "0 ")?;
                    } else {
                        write!(writer, "1 ")?;
                    }
                }
                write!(writer, "0 1\n")?;
                write!(writer, "1 ")?;
                for x in 0..self.width {
                    if (self.predecessor[(y + 1) * self.width + x] == (y * self.width + x) as i32)
                        || (self.predecessor[y * self.width + x]
                            == ((y + 1) * self.width + x) as i32)
                    {
                        write!(writer, "0 ")?;
                    } else {
                        write!(writer, "1 ")?;
                    }
                    write!(writer, "1")?;
                    if x != self.width -1 {
                        write!(writer, " ")?;
                    }
                }
                write!(writer, "\n")?;
            }
            write!(writer, "1 ")?;
            for x in 0..self.width - 1 {
                write!(writer, "0 ")?;
                if (self.predecessor[((self.height - 1) * self.width) + x + 1]
                    == (((self.height - 1) * self.width) + x) as i32)
                    || (self.predecessor[((self.height - 1) * self.width) + x]
                        == (((self.height - 1) * self.width) + x + 1) as i32)
                {
                    write!(writer, "0 ")?;
                } else {
                    write!(writer, "1 ")?;
                }
            }
            write!(writer, "0 1\n")?;
            write!(writer, "{}", &"1 ".repeat(2 * self.width))?;
            write!(writer, "1")?;
            Ok(())
        }
    }

    impl std::fmt::Display for Maze {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}\n", &"⬛".repeat(2 * self.width + 1))?;

            for y in 0..self.height - 1 {
                write!(f, "⬛")?;
                for x in 0..self.width - 1 {
                    write!(f, "⬜")?;
                    if (self.predecessor[(y * self.width) + x + 1] == ((y * self.width) + x) as i32)
                        || (self.predecessor[(y * self.width) + x]
                            == ((y * self.width) + x + 1) as i32)
                    {
                        write!(f, "⬜")?;
                    } else {
                        write!(f, "⬛")?;
                    }
                }

                write!(f, "⬜⬛\n⬛")?;

                for x in 0..self.width {
                    if (self.predecessor[(y + 1) * self.width + x] == (y * self.width + x) as i32)
                        || (self.predecessor[y * self.width + x]
                            == ((y + 1) * self.width + x) as i32)
                    {
                        write!(f, "⬜")?;
                    } else {
                        write!(f, "⬛")?;
                    }
                    write!(f, "⬛")?;
                }

                write!(f, "\n")?;
            }
            write!(f, "⬛")?;
            for x in 0..self.width - 1 {
                write!(f, "⬜")?;
                if (self.predecessor[((self.height - 1) * self.width) + x + 1]
                    == (((self.height - 1) * self.width) + x) as i32)
                    || (self.predecessor[((self.height - 1) * self.width) + x]
                        == (((self.height - 1) * self.width) + x + 1) as i32)
                {
                    write!(f, "⬜")?;
                } else {
                    write!(f, "⬛")?;
                }
            }
            write!(f, "⬜⬛")?;
            write!(f, "\n{}", &"⬛".repeat(2 * self.width + 1))
        }
    }
}
