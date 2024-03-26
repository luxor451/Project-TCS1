//! Main library containing most of the implementation for Maps, Neighbors, Maze and a BinaryHeap implementation
//! Also implement a Matrix<_> struct

pub mod map_load {
    use std::fmt;

    pub struct Edges {
        pub down: i64,
        pub right: i64,
    }
    pub struct Map {
        pub width: usize,
        pub height: usize,
        pub edges_matrix: Matrix<Edges>,
    }

    /// Represents a matrix of elements of type `T`.
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
        pub fn new(vec: Vec<T>, n_col: usize, n_row: usize) -> Self {
            assert!(vec.len() == n_row * n_col);
            return Self {
                vec: vec,
                row: n_row,
                col: n_col,
            };
        }

        /// Returns a slice of the matrix representing the specified row.
        pub fn row(&self, row: usize) -> &[T] {
            let i = self.col * row;
            return &self.vec[i..(i + self.col)];
        }

        /// Returns a reference to the element at the specified coordinates.
        pub fn get(&self, x: usize, y: usize) -> &T {
            let i = self.col * y;
            return &self.vec[i + x];
        }

        /// Returns a mutable reference to the element at the specified coordinates.
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

pub mod neighors {
    use crate::core::map_load::*;
    use std::fmt;

    pub struct Neighbors {
        pub nb: usize,
        pub neighbors_names: [i64; 4],
        pub edges_cost: [i64; 4],
    }

    impl Neighbors {
        /// Return a node with all information set to 0
        pub fn empty_node() -> Self {
            return Self {
                nb: 0,
                neighbors_names: [0, 0, 0, 0],
                edges_cost: [0, 0, 0, 0],
            };
        }
        // Return the neighbors of the node in position (x, y) in map
        pub fn new_node(map: &Map, x: usize, y: usize) -> Self {
            let mut nb: usize = 0;
            let mut neighbors_names: [i64; 4] = [-1, -1, -1, -1];
            let mut edges_cost: [i64; 4] = [0, 0, 0, 0];
            let curent_node: i64 = (map.width * y + x) as i64;
            // Iterate throught each case and if the neighbors exist add it to the arrays;
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
                neighbors_names[2] = curent_node - map.width as i64;
                edges_cost[2] = map.edges_matrix.get(x, y - 1).down;
            }
            if y < map.height - 1 {
                nb = nb + 1;
                neighbors_names[0] = curent_node + map.width as i64;
                edges_cost[0] = map.edges_matrix.get(x, y).down;
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

pub mod maze {
    use std::io::Write;
    use std::{fmt, fs::File, io::BufWriter};

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

pub mod binary_heap {
    macro_rules! last {
        ($vec: expr) => {
            $vec.last().unwrap().borrow()
        };
    }
    macro_rules! right_child {
        ($vec: expr) => {
            last!($vec).right_child.as_ref().unwrap()
        };
    }
    macro_rules! left_child {
        ($vec: expr) => {
            last!($vec).left_child.as_ref().unwrap()
        };
    }
    use std::{
        cell::{RefCell, RefMut},
        fmt::Debug,
        mem::swap,
        rc::Rc,
    };

    #[derive(Debug)]
    pub struct Node<T> {
        pub value: T,
        pub cost: i64,
        pub left_child: Option<Rc<NodeCell<T>>>,
        pub right_child: Option<Rc<NodeCell<T>>>,
    }

    // For the sake of readability
    pub type NodeCell<T> = RefCell<Node<T>>;

    #[derive(Debug)]
    pub struct Heap<T> {
        pub size: usize,
        pub root: Option<Rc<NodeCell<T>>>,
        pub pos: Vec<Rc<NodeCell<T>>>,
    }
    impl<T: Default> Default for Node<T> {
        /// Implement a deufalt value for Node : value = T::default(), cost = 0, left_child = None, right_child = None
        fn default() -> Self {
            Node {
                value: T::default(),
                cost: 0,
                left_child: None,
                right_child: None,
            }
        }
    }

    impl<T: Debug + Default + Clone> Heap<T> {
        /// Return an empty heap
        pub fn default() -> Self {
            return Self {
                size: 0,
                root: None,
                pos: Vec::new(),
            };
        }
        /// Return true if the heap is empty
        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        /// Return the path to the father of the node n
        pub fn path_to_father_of_node(&self, n: usize) -> Option<Vec<Rc<NodeCell<T>>>> {
            let depth: usize = n.ilog2() as usize;
            let heap_depth: usize = self.size.ilog2() as usize;
            if depth == 0 {
                return None;
            }
            // Initilaze the Vec at heap_depth in cas we want to modifie it in another function, we don't want to reallocate
            // Since the depht is at most log2(size) we can allocate it at this size without any problem, only a few elements will be unused
            let mut path: Vec<_> = Vec::with_capacity(heap_depth);
            path.push(self.root.as_ref()?.clone());

            // For the most part it's the algorith given in the subject
            for i in (1..depth).rev() {
                let node: Rc<NodeCell<T>> = if ((n >> i) & 1) == 0 {
                    path.last()?.borrow().left_child.as_ref()?.clone()
                } else {
                    path.last()?.borrow().right_child.as_ref()?.clone()
                };
                path.push(node);
            }
            return Some(path);
        }

        /// Given a new node and a path to the father of the node, balance the heap so that it conserve the heap property
        pub fn heapify_up(
            &mut self,
            new_node: Rc<NodeCell<T>>,
            mut path: Vec<Rc<NodeCell<T>>>,
        ) -> usize {
            path.push(new_node.clone());
            let cost = new_node.borrow().cost;

            let mut n = self.size;
            // If path.len() <=, that means that the node is the root and thus we don't need to do anything
            if path.len() > 1 {
                let mut current_index: usize = path.len() - 1;
                let mut current_node: RefMut<'_, Node<T>> = path[current_index].borrow_mut();
                let mut father_cost: i64 = path[current_index - 1].borrow().cost;
                while (current_index >= 1) && (father_cost > cost) {
                    let father: Rc<NodeCell<T>> = path[current_index - 1].clone();
                    swap(&mut current_node.cost, &mut father.borrow_mut().cost);
                    swap(&mut current_node.value, &mut father.borrow_mut().value);
                    current_index -= 1;
                    current_node = path[current_index].borrow_mut();
                    n = n >> 1;
                    if current_index > 0 {
                        father_cost = path[current_index - 1].borrow().cost;
                    }
                }
            }
            return n;
        }

        /// Bubble down the new root to the right place so that the heap property is conserved
        // Honesty time : Antonin who is currently in 2A and had done a lot of Rust at the ENS last year helped me a lot with this function.
        // I was not familiar enough with RefCells and Rc to do it by myself at the time
        // the issues is that since you can only have one mutable reference to a RefCell at a time and variable have very limited lifetime.
        // The solution that we found was to get evrything in a Vec that would outlive every other variable and only get a node from the Ref that was in the Vec
        // and not use intermediate variables that would not live long enough thus giving this not so readable code
        pub fn heapify_down(&self) -> usize {
            let depth: usize = self.size.ilog2() as usize;
            let mut path_to_new_pos: Vec<Rc<NodeCell<T>>> = Vec::with_capacity(depth);
            let cost = self.root.as_ref().unwrap().borrow().cost;
            path_to_new_pos.push(self.root.as_ref().unwrap().clone());
            let mut n = 1;

            // Construct the path to the new position
            // While the last node has a right child and the cost is greater than one of the children
            while (last!(path_to_new_pos).right_child.is_some())
                && (cost > right_child!(path_to_new_pos).borrow().cost
                    || cost > left_child!(path_to_new_pos).borrow().cost)
            {
                // Get the child with the smallest cost

                let node = if left_child!(path_to_new_pos).borrow().cost
                    < right_child!(path_to_new_pos).borrow().cost
                {
                    n = n << 1;
                    left_child!(path_to_new_pos).clone()
                } else {
                    n = (n << 1) + 1;
                    right_child!(path_to_new_pos).clone()
                };
                // Push the new node in the Vec

                path_to_new_pos.push(node);
            }

            // If we're in a postion where the Node as a left child and  not a right child
            if last!(path_to_new_pos).left_child.is_some()
                && left_child!(path_to_new_pos).borrow().cost < cost
            {
                let node = left_child!(path_to_new_pos).clone();
                path_to_new_pos.push(node);
            }

            // Swap the cost and value of the nodes
            for i in 0..(path_to_new_pos.len() - 1) {
                swap(
                    &mut path_to_new_pos[i].borrow_mut().cost,
                    &mut path_to_new_pos[i + 1].borrow_mut().cost,
                );
                swap(
                    &mut path_to_new_pos[i].borrow_mut().value,
                    &mut path_to_new_pos[i + 1].borrow_mut().value,
                );
            }
            return n;
        }

        /// Insert new_node in the heap and balance it such it conserve the heap property
        pub fn insert(&mut self, new_node: Rc<NodeCell<T>>) -> usize {
            // To note: if a node as a right child in a heap, it implies that it has a left child
            let new_node = new_node;
            if self.is_empty() {
                self.size = 1;
                self.root = Some(new_node);
                return 0;
            } else {
                // Insert in the last position
                let path = self.path_to_father_of_node(self.size + 1).unwrap();
                let father_of_new_node = path.last().unwrap();
                // If it has a left child, the new_node become the right child
                if father_of_new_node.borrow().left_child.is_some() {
                    assert!(father_of_new_node.borrow().right_child.is_none());
                    father_of_new_node.borrow_mut().right_child = Some(new_node.clone());
                } else {
                    father_of_new_node.borrow_mut().left_child = Some(new_node.clone());
                }
                self.size += 1;
                return self.heapify_up(new_node, path);
            }
        }

        /// Extract the minimum value of the heap and balance it such it conserve the heap property
        pub fn extract_min(&mut self) -> (Option<T>, usize) {
            // Base case:
            // Empty heap
            if self.size == 0 {
                return (None, 0);
            }
            // Heap with just a root, return the value of the root and make the heap empty
            if self.size == 1 {
                let res = self.root.as_ref().take().unwrap().borrow().value.clone();
                self.root = None;
                self.size -= 1;
                return (Some(res), 1);
            }
            let path = self.path_to_father_of_node(self.size).unwrap();
            // If it had between 2 and 3 nodes, we return the root and make the smallest node the root, we have to do this because if the last node and the father of the last node are the same
            // beacuse root and father_of_last_node defined later will be 2 RefMut to the same RefCell and thus a runtime error
            // In this implementation it his done by destroying (with .take()) then re-constructing the heap
            let mut new_pos_of_last_elem = 1;
            if path.len() == 1 {
                let root = self.root.as_ref().take().unwrap().borrow_mut();
                let res = root.value.clone();
                let mut new_root = Rc::new(RefCell::new(Node::default()));
                if root.right_child.is_some() {
                    let right_child = root.right_child.as_ref().take().unwrap();
                    let left_child = root.left_child.as_ref().take().unwrap();
                    if right_child.borrow().cost < left_child.borrow().cost {
                        right_child.borrow_mut().left_child = Some(left_child.clone());
                        new_root = right_child.clone();
                        new_pos_of_last_elem = 2;
                    } else {
                        left_child.borrow_mut().left_child = Some(right_child.clone());
                        new_root = left_child.clone();
                        new_pos_of_last_elem = 1;
                    }
                } else {
                    if root.left_child.is_some() {
                        new_root = root.left_child.as_ref().take().unwrap().clone();
                    }
                }
                self.size -= 1;
                drop(root);
                self.root = Some(new_root);
                return (Some(res), new_pos_of_last_elem);
            }

            let mut root = path[0].borrow_mut();
            let mut father_of_last_node = path.last().unwrap().borrow_mut();
            let res = Some(root.value.clone());
            // Remove the last node
            match father_of_last_node.right_child {
                Some(ref node) => {
                    root.cost = node.borrow().cost;
                    root.value = node.borrow().value.clone();
                    father_of_last_node.right_child = None;
                }
                None => {
                    let left_child = father_of_last_node.left_child.as_ref().unwrap().clone();
                    root.cost = left_child.borrow().cost;
                    root.value = left_child.borrow().value.clone();
                    father_of_last_node.left_child = None;
                }
            }
            self.size -= 1;
            // Destroy every posible RefMut that might be in the path to give complete ownership to heapify_down
            drop(father_of_last_node);
            drop(root);
            drop(path);
            new_pos_of_last_elem = self.heapify_down();
            return (res, new_pos_of_last_elem);
        }

        pub fn change_cost(&mut self, new_node: Rc<NodeCell<T>>, n: usize, new_cost: i64) -> () {
            new_node.as_ref().borrow_mut().cost = new_cost;
            self.heapify_up(new_node, self.path_to_father_of_node(n).unwrap());
        }
    }

    impl<T: PartialEq> PartialEq for Node<T> {
        /// Define equality for Node : two nodes are equal iff they have the same value, cost, left_child and right_child
        fn eq(&self, other: &Self) -> bool {
            let mut res = self.cost == other.cost;
            res &= self.value == other.value;
            res &= self.left_child == self.left_child;
            res &= self.right_child == self.right_child;
            res
        }
    }
}
