pub mod map_load {
    use std::fmt;

    pub struct Edges  {
        pub down : i32,
        pub right : i32,
    }

    

    pub struct Map {
        pub width : usize,
        pub height : usize,
        pub edges_matrix : Matrix<Edges>
    }

 

    #[derive(Debug)]
    pub struct Matrix<T> {
        pub vec: Vec<T>,
        pub row: usize,
        pub col: usize,
    }

    impl<T> Matrix<T> {
        pub fn new(vec: Vec<T>, n_col: usize, n_row: usize) -> Self {
            print!("{}\n", vec.len());
            assert!(vec.len() == n_row * n_col);
            Self { vec : vec, row : n_row, col :n_col }
        }

        pub fn row(&self, row: usize) -> &[T] {
            let i = self.col * row;
            &self.vec[i..(i + self.col)]
        }

        pub fn get(&self,n_row: usize, n_col: usize) -> &T {
            let i = self.col * n_row;
            &self.vec[i + n_col]
        }

        pub fn get_mut(&mut self, n_row: usize, n_col: usize) -> &mut T {
            let i = self.col * n_row;
            &mut self.vec[i + n_col]
        }
    }
    
    impl<T: std::fmt::Debug> std::fmt::Display for Matrix<T> {
        
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut str = String::new();
            for i in 0..self.row {
                if i != 0 {
                    str.push_str(", ");
                }
                str.push_str(&format!("{:?}", &self.row(i)));
            }
            write!(f, "[{}]", str)
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