//! Contains the implementation for Matrix and Map structs.

/// Traits for Matrix are : new, row, get, get_mut
pub mod map_load {

    use std::fmt;

    /// Represents the edges of a node in a map.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new Edges struct
    /// let edges = Edges {
    ///     down: 4,
    ///     right: 2,
    /// };
    /// ```
    pub struct Edges {
        pub down: i64,
        pub right: i64,
    }

    /// Represents a map with a matrix of edges.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new Matrix of Edges
    /// let edges_matrix = Matrix {
    ///     vec: vec![
    ///         Edges { down: 4, right: 2 },
    ///         Edges { down: 5, right: 3 },
    ///     ],
    ///     row: 2,
    ///     col: 1,
    /// };
    ///
    /// // Create a new Map
    /// let map = Map {
    ///     width: 1,
    ///     height: 2,
    ///     edges_matrix,
    /// };
    /// ```
    pub struct Map {
        pub width: usize,
        pub height: usize,
        pub edges_matrix: Matrix<Edges>,
    }

    /// Represents a matrix of elements of type `T`.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new Matrix of Edges
    /// let matrix = Matrix {
    ///     vec: vec![
    ///         Edges { down: 4, right: 2 },
    ///         Edges { down: 5, right: 3 },
    ///     ],
    ///     row: 2,
    ///     col: 1,
    /// };
    /// ```
    #[derive(Debug)]
    pub struct Matrix<T> {
        pub vec: Vec<T>,
        pub row: usize,
        pub col: usize,
    }

    impl<T> Matrix<T> {
        /// Creates a new matrix with the given vector, number of columns, and number of rows.
        ///
        /// # Panics
        ///
        /// Panics if the length of the vector is not equal to `n_col * n_row`.
        ///
        /// # Example
        ///
        /// ```
        /// // Create a new Matrix of Edges using the new function
        /// let matrix = Matrix::new(
        ///     vec![
        ///         Edges { down: 4, right: 2 },
        ///         Edges { down: 5, right: 3 },
        ///     ],
        ///     1, // number of columns
        ///     2, // number of rows
        /// );
        /// ```
        pub fn new(vec: Vec<T>, n_col: usize, n_row: usize) -> Self {
            assert!(vec.len() == n_row * n_col);
            return Self {
                vec: vec,
                row: n_row,
                col: n_col,
            };
        }
        /// Returns a slice of the matrix representing the specified row.
        /// # Example
        ///
        /// ```
        /// // Create a new Matrix of Edges using the new function
        /// let matrix = Matrix::new(
        ///     vec![
        ///         Edges { down: 4, right: 2 },
        ///         Edges { down: 5, right: 3 },
        ///     ],
        ///     1, // number of columns
        ///     2, // number of rows
        /// );
        ///
        /// // Get the first row of the matrix
        /// let row = matrix.row(0);
        /// assert_eq!(row, vec![Edges { down: 4, right: 2 }]);
        /// ```
        /// Now, row is a slice of the matrix representing the first row.
        pub fn row(&self, row: usize) -> &[T] {
            let i = self.col * row;
            return &self.vec[i..(i + self.col)];
        }

        /// Returns a reference to the element at the specified coordinates.
        ///
        /// # Example
        ///
        /// ```
        /// // Create a new Matrix of Edges using the new function
        /// let matrix = Matrix::new(
        ///     vec![
        ///         Edges { down: 4, right: 2 },
        ///         Edges { down: 5, right: 3 },
        ///     ],
        ///     1, // number of columns
        ///     2, // number of rows
        /// );
        ///
        /// // Get the element at coordinates (0, 1)
        /// let element = matrix.get(0, 1);
        /// assert_eq!(element, &Edges { down: 5, right: 3 });
        /// ```
        pub fn get(&self, x: usize, y: usize) -> &T {
            let i = self.col * y;
            return &self.vec[i + x];
        }

        /// Returns a mutable reference to the element at the specified coordinates.
        ///
        /// /// # Example
        ///
        /// ```
        /// // Create a new Matrix of Edges using the new function
        /// let mut matrix = Matrix::new(
        ///     vec![
        ///         Edges { down: 4, right: 2 },
        ///         Edges { down: 5, right: 3 },
        ///     ],
        ///     1, // number of columns
        ///     2, // number of rows
        /// );
        ///
        /// // Get a mutable reference to the element at coordinates (0, 1)
        /// let element = matrix.get_mut(0, 1);
        /// *element = Edges { down: 6, right: 4 };
        /// assert_eq!(element, &Edges { down: 6, right: 4 });
        /// ```
        /// The element at the specified coordinates in the matrix is now `Edges { down: 6, right: 4 }`.
        pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
            let i = self.col * x;
            return &mut self.vec[i + y];
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Display for Matrix<T> {
        /// Formats the matrix for display.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, " [\n")?;
            for i in 0..self.row {
                if i != 0 {
                    write!(f, ",\n ")?;
                }
                write!(f, "{}", &format!("{:?}", &self.row(i)))?;
            }
            write!(f, "\n]")
        }
    }

    impl std::fmt::Display for Edges {
        /// Formats the edges for display.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(down : {} ; right : {})", self.down, self.right)
        }
    }

    impl std::fmt::Debug for Edges {
        /// Formats the edges for debugging.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(down : {} ; right : {})", self.down, self.right)
        }
    }
}
