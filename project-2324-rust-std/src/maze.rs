//! Contains the implementation and tests for mazes.

/// Traits for Maze are : write_maze_in_pbm
pub mod maze {

    use std::io::Write;
    use std::{fmt, fs::File, io::BufWriter};

    /// Represents a maze with a array of the predessesor of each node in the MST
    ///
    /// # Example
    ///  ```
    ///  // Representation of the maze_10_8_2048 given in the data
    /// let maze_array_2048: Vec<i64> = vec![
    ///       -1, 0, 3, 13, 14, 4, 7, 17, 18, 19, 11, 1, 22, 12, 13, 14, 15, 18, 28, 29, 10, 20, 21,
    ///       22, 34, 26, 16, 26, 38, 28, 20, 21, 31, 32, 44, 25, 37, 27, 37, 38, 50, 51, 32, 42, 43,
    ///       46, 36, 37, 58, 59, 51, 52, 42, 63, 53, 56, 46, 47, 59, 69, 61, 62, 52, 73, 54, 64, 65,
    ///       66, 67, 79, 60, 72, 62, 72, 75, 65, 66, 78, 68, 78,
    /// ];
    /// let maze_2048: Maze = Maze {
    ///    width: 10,
    ///    height: 8,
    ///    predecessor: maze_array_2048,
    ///    cost: -666,
    /// };
    /// ```
    pub struct Maze {
        pub width: usize,
        pub height: usize,
        pub predecessor: Vec<i64>,
        pub cost: i64,
    }

    impl Maze {
        /// Create a pbm file named print_maze.pbm
        /// Write in this file the maze.
        pub fn write_maze_in_pbm(&self) -> std::io::Result<()> {
            let file = File::create("print_maze.pbm")?;
            // First row of wall
            let mut writer: BufWriter<&File> = BufWriter::new(&file);
            write!(
                writer,
                "P1\n{} {}\n",
                2 * self.width + 1,
                2 * self.height + 1
            )?;
            write!(writer, "{}", &"1 ".repeat(2 * self.width))?;
            write!(writer, "1\n")?;

            // For each row : first print out the horizontal connection
            // and then the vertical connection
            for y in 0..self.height - 1 {
                // left wall
                write!(writer, "1 ")?;
                // horizontal connection
                for x in 0..self.width - 1 {
                    // The  node itself
                    write!(writer, "0 ")?;
                    // If the next node is the predecessor of the current node or inversly
                    if (self.predecessor[(y * self.width) + x + 1] == ((y * self.width) + x) as i64)
                        || (self.predecessor[(y * self.width) + x]
                            == ((y * self.width) + x + 1) as i64)
                    {
                        write!(writer, "0 ")?;
                    } else {
                        write!(writer, "1 ")?;
                    }
                }
                // Last node of the row
                write!(writer, "0 1\n")?;
                // Vertical connection
                // Left wall
                write!(writer, "1 ")?;
                for x in 0..self.width {
                    // If the node below is the predecessor of the current node or inversly
                    if (self.predecessor[(y + 1) * self.width + x] == (y * self.width + x) as i64)
                        || (self.predecessor[y * self.width + x]
                            == ((y + 1) * self.width + x) as i64)
                    {
                        write!(writer, "0 ")?;
                    } else {
                        write!(writer, "1 ")?;
                    }
                    // Under a horizontal connection, there is always a wall
                    write!(writer, "1")?;
                    if x != self.width - 1 {
                        write!(writer, " ")?;
                    }
                }
                write!(writer, "\n")?;
            }
            // Last row
            write!(writer, "1 ")?;
            for x in 0..self.width - 1 {
                write!(writer, "0 ")?;
                if (self.predecessor[((self.height - 1) * self.width) + x + 1]
                    == (((self.height - 1) * self.width) + x) as i64)
                    || (self.predecessor[((self.height - 1) * self.width) + x]
                        == (((self.height - 1) * self.width) + x + 1) as i64)
                {
                    write!(writer, "0 ")?;
                } else {
                    write!(writer, "1 ")?;
                }
            }
            // End of row
            write!(writer, "0 1\n")?;
            // South wall
            write!(writer, "{}", &"1 ".repeat(2 * self.width))?;
            write!(writer, "1")
        }
    }

    impl std::fmt::Display for Maze {
        /// Format a Maze such that it is displayed in the console
        // Implementation details : same algorithm as for pbm files
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}\n", &"⬛".repeat(2 * self.width + 1))?;
            for y in 0..self.height - 1 {
                write!(f, "⬛")?;
                for x in 0..self.width - 1 {
                    write!(f, "⬜")?;
                    if (self.predecessor[(y * self.width) + x + 1] == ((y * self.width) + x) as i64)
                        || (self.predecessor[(y * self.width) + x]
                            == ((y * self.width) + x + 1) as i64)
                    {
                        write!(f, "⬜")?;
                    } else {
                        write!(f, "⬛")?;
                    }
                }

                write!(f, "⬜⬛\n⬛")?;

                for x in 0..self.width {
                    if (self.predecessor[(y + 1) * self.width + x] == (y * self.width + x) as i64)
                        || (self.predecessor[y * self.width + x]
                            == ((y + 1) * self.width + x) as i64)
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
                    == (((self.height - 1) * self.width) + x) as i64)
                    || (self.predecessor[((self.height - 1) * self.width) + x]
                        == (((self.height - 1) * self.width) + x + 1) as i64)
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

#[cfg(test)]

mod test_maze {
    use crate::maze::maze::Maze;

    #[test]
    fn test_print_maze() {
        print!("print_maze test on maze_2048 :\n");
        let maze_array_2048: Vec<i64> = vec![
            -1, 0, 3, 13, 14, 4, 7, 17, 18, 19, 11, 1, 22, 12, 13, 14, 15, 18, 28, 29, 10, 20, 21,
            22, 34, 26, 16, 26, 38, 28, 20, 21, 31, 32, 44, 25, 37, 27, 37, 38, 50, 51, 32, 42, 43,
            46, 36, 37, 58, 59, 51, 52, 42, 63, 53, 56, 46, 47, 59, 69, 61, 62, 52, 73, 54, 64, 65,
            66, 67, 79, 60, 72, 62, 72, 75, 65, 66, 78, 68, 78,
        ];
        let maze_2048: Maze = Maze {
            width: 10,
            height: 8,
            predecessor: maze_array_2048,
            cost: -666,
        };
        println!("{}", maze_2048);
    }

    #[test]
    fn test_maze_in_pbm() {
        print!("pbm test on maze_2048 :\n");
        let maze_array_2048: Vec<i64> = vec![
            -1, 0, 3, 13, 14, 4, 7, 17, 18, 19, 11, 1, 22, 12, 13, 14, 15, 18, 28, 29, 10, 20, 21,
            22, 34, 26, 16, 26, 38, 28, 20, 21, 31, 32, 44, 25, 37, 27, 37, 38, 50, 51, 32, 42, 43,
            46, 36, 37, 58, 59, 51, 52, 42, 63, 53, 56, 46, 47, 59, 69, 61, 62, 52, 73, 54, 64, 65,
            66, 67, 79, 60, 72, 62, 72, 75, 65, 66, 78, 68, 78,
        ];
        let maze_2048: Maze = Maze {
            width: 10,
            height: 8,
            predecessor: maze_array_2048,
            cost: -666,
        };
        let _ = Maze::write_maze_in_pbm(&maze_2048);
    }
}
