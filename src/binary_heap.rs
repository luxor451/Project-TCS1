//! Contains the implementation and tests for binary heaps.
/// Traits for binary heaps are : default, is_empty, insert_node, extract_min
pub mod binary_heap {

    // Macros for code readability
    /// Return the last element of a vector
    macro_rules! last {
        ($vec: expr) => {
            $vec.last().unwrap().borrow()
        };
    }
    /// Return the right child of the last element of a vector
    macro_rules! right_child {
        ($vec: expr) => {
            last!($vec).right_child.as_ref().unwrap()
        };
    }
    /// Return the left child of the last element of a vector
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
    /// A node in a binary heap.
    ///
    /// Each node has a value, a cost, and optional left and right children.
    ///
    /// # Example
    ///
    /// ```
    /// let node = Node {
    ///     value: "value",
    ///     cost: 42,
    ///     left_child: None,
    ///     right_child: None,
    /// };
    /// ```
    /// In this example, a new `Node` is created with a value of `"value"`, a cost of `42`, and no children.
    #[derive(Debug)]
    pub struct Node<T> {
        pub value: T,
        pub cost: i64,
        pub left_child: Option<Rc<NodeCell<T>>>,
        pub right_child: Option<Rc<NodeCell<T>>>,
    }

    // For the sake of readability
    pub type NodeCell<T> = RefCell<Node<T>>;

    /// A binary heap data structure.
    ///
    /// Each heap has a size and a `Refcell` to a root node.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new Node
    /// let node = Node {
    ///     value: "value",
    ///     cost: 42,
    ///     left_child: None,
    ///     right_child: None,
    /// };
    ///
    /// // Wrap the Node in a NodeCell and an Rc
    /// let node_cell = Rc::new(RefCell::new(node));
    ///
    /// // Create a new Heap
    /// let heap = Heap {
    ///     size: 1,
    ///     root: Some(Rc::clone(&node_cell)),
    /// };
    /// ```
    /// Now, heap is a Heap that contains one `Node` with a value of `"value"`, a cost of `42`, and no children.
    #[derive(Debug)]
    pub struct Heap<T> {
        pub size: usize,
        pub root: Option<Rc<NodeCell<T>>>,
    }
    impl<T: Default> Default for Node<T> {
        /// Implement a default value for Node : value = T::default(), cost = 0, left_child = None, right_child = None
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
        ///
        /// # Exemple
        ///
        /// ```
        /// let heap = Heap::default();
        ///
        /// // Now, heap is an empty Heap.
        /// assert_eq!(heap.size, 0);
        /// assert!(heap.root.is_none());
        /// ```
        pub fn default() -> Self {
            return Self {
                size: 0,
                root: None,
            };
        }
        /// Return true iff the heap is empty
        /// # Exemple
        ///
        /// ```
        /// let heap = Heap::default();
        ///
        /// // Now, heap is an empty Heap.
        /// assert(heap.is_empty());
        /// assert!(heap.root.is_none());
        /// ```
        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        /// Return the path to the father of the node n if n >= 1 else return None
        ///
        /// # Exemple
        ///
        /// ```
        /// // Create a new Node
        /// let child_1 = Node {
        ///     value: "child1",
        ///     cost: 100,
        ///     left_child: None,
        ///     right_child: None,
        /// };
        ///
        ///let child_2 = Node {
        ///     value: "child2",
        ///     cost: 1000,
        ///     left_child: None,
        ///     right_child: None,
        /// };
        /// let child_cell_1 = Rc::new(RefCell::new(child_1));
        /// let child_cell_2 = Rc::new(RefCell::new(child_2));
        ///
        /// let node = Node {
        ///     value: "father",
        ///     cost: 42,
        ///     left_child: Some(child_1),
        ///     right_child: Some(child_2),
        /// };
        ///
        /// // Wrap the Nodes in a NodeCell and an Rc
        /// let node_cell = Rc::new(RefCell::new(node));
        ///
        /// // Create a new Heap
        /// let mut heap = Heap {
        ///     size: 1,
        ///     root: Some(Rc::clone(&node_cell)),
        /// };
        ///
        /// // Get the path to the father of the node at position 1
        /// let path = heap.path_to_father_of_node(2);
        /// ```
        /// The path should be `[Some(nodecell)]` as it is the father of the node number 2 which is the left child of the root
        pub fn path_to_father_of_node(&self, n: usize) -> Option<Vec<Rc<NodeCell<T>>>> {
            let depth: usize = n.ilog2() as usize;
            let heap_depth: usize = self.size.ilog2() as usize;
            if depth == 0 {
                return None;
            }
            // Initilaze the Vec at heap_depth in case we want to modifie it in another function, we don't want to reallocate
            // Since the depht is at most log2(size) we can allocate it at this size without any problem, only a few elements will be unused
            let mut path: Vec<_> = Vec::with_capacity(heap_depth);
            path.push(self.root.as_ref()?.clone());

            // For the most part it's the algorithm given in the subject
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
        fn heapify_up(&mut self, new_node: Rc<NodeCell<T>>, mut path: Vec<Rc<NodeCell<T>>>) {
            path.push(new_node.clone());
            let cost = new_node.borrow().cost;

            // If path.len() <=, that means that the node is the root and thus we don't need to do anything
            if path.len() > 1 {
                let mut current_index: usize = path.len() - 1;
                let mut current_node: RefMut<'_, Node<T>> = path[current_index].borrow_mut();
                let mut father_cost: i64 = path[current_index - 1].borrow().cost;
                // While we are not on the top of the tree and the cost is lower than the father's cost
                while (current_index >= 1) && (father_cost > cost) {
                    let father: Rc<NodeCell<T>> = path[current_index - 1].clone();
                    swap(&mut current_node.cost, &mut father.borrow_mut().cost);
                    swap(&mut current_node.value, &mut father.borrow_mut().value);
                    current_index -= 1;
                    current_node = path[current_index].borrow_mut();

                    if current_index > 0 {
                        father_cost = path[current_index - 1].borrow().cost;
                    }
                }
            }
        }

        /// Bubble down the new root to the right place so that the heap property is conserved
        // Honesty time : Antonin who is currently in 2A and had done a lot of Rust at the ENS last year helped me a lot with this function.
        // I was not familiar enough with RefCells and Rc to do it by myself at the time
        // the issues is that since you can only have one mutable reference to a RefCell at a time and variable have very limited lifetime.
        // The solution that we found was to get evrything in a Vec that would outlive every other variable and only get a node from the Ref that was in the Vec
        // and not use intermediate variables that would not live long enough thus giving this not so readable code
        fn heapify_down(&self) {
            let depth: usize = self.size.ilog2() as usize;
            let mut path_to_new_pos: Vec<Rc<NodeCell<T>>> = Vec::with_capacity(depth);
            let cost = self.root.as_ref().unwrap().borrow().cost;
            path_to_new_pos.push(self.root.as_ref().unwrap().clone());

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
                    left_child!(path_to_new_pos).clone()
                } else {
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
        }

        /// Insert new_node in the heap and balance it such a way it conserve the heap property
        ///
        /// # Exemple
        ///
        /// ```
        /// // Create a new Node
        /// let new_node = Node {
        ///     value: "value",
        ///     cost: 42,
        ///     left_child: None,
        ///     right_child: None,
        /// };
        ///
        /// // Wrap the Node in a NodeCell and an Rc
        /// let new_node_cell = Rc::new(RefCell::new(new_node));
        ///
        /// // Create a new Heap
        /// let mut heap = Heap {
        ///    size: 1,
        ///    root: None,
        /// };
        ///
        /// // Insert the new node in the heap
        /// heap.insert(new_node_cell);
        /// ```
        /// Now, heap is a Heap that contains one `Node` with a value of `"value"`, a cost of `42`, and no children.
        pub fn insert(&mut self, new_node: Rc<NodeCell<T>>) {
            // To note: if a node as a right child in a heap, it implies that it has a left child
            if self.is_empty() {
                self.size = 1;
                self.root = Some(new_node);
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
                self.heapify_up(new_node, path);
            }
        }

        /// Extract the value with the minimun cost of the heap and balance it such a way it conserve the heap property
        ///
        /// # Exemple
        ///
        /// ```
        ///
        /// let mut new_heap: Heap<i64> = Heap::default();
        /// // Insert 10 nodes in the heap in the reverse order (any order should work)
        /// for i in (0..10).rev() {
        ///    new_heap.insert(Rc::new(RefCell::new(Node {
        ///        value: i,
        ///        cost: i as i64,
        ///        left_child: None,
        ///        right_child: None,
        /// })));}
        ///
        /// let min = new_heap.extract_min();
        ///
        /// asserteq!(min.unwrap(), 0);
        /// ```
        /// The min of `new_heap` for the cost is `0`;
        pub fn extract_min(&mut self) -> Option<T> {
            // Base case:
            // Empty heap
            if self.size == 0 {
                return None;
            }
            // Heap with just a root, return the value of the root and make the heap empty
            if self.size == 1 {
                let res = self.root.as_ref().take().unwrap().borrow().value.clone();
                self.root = None;
                self.size -= 1;
                return Some(res);
            }
            let path = self.path_to_father_of_node(self.size).unwrap();
            // If it had between 2 and 3 nodes, we return the root and make the smallest node the root, we have to do this because if the last node and the father of the last node are the same
            // beacuse root and father_of_last_node defined later will be 2 RefMut to the same RefCell and thus a runtime error
            // In this implementation it his done by destroying (with .take()) then re-constructing the heap

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
                    } else {
                        left_child.borrow_mut().left_child = Some(right_child.clone());
                        new_root = left_child.clone();
                    }
                } else {
                    if root.left_child.is_some() {
                        new_root = root.left_child.as_ref().take().unwrap().clone();
                    }
                }
                self.size -= 1;
                drop(root);
                self.root = Some(new_root);
                return Some(res);
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
            // Destroy every posible RefMu to give complete ownership to heapify_down
            // This is not idiomatic Rust but it works
            drop(father_of_last_node);
            drop(root);
            drop(path);
            self.heapify_down();
            return res;
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

#[cfg(test)]
mod test_bheap {
    macro_rules! value {
        ($vec: expr) => {
            $vec.unwrap().last().unwrap().borrow().value
        };
    }

    use crate::binary_heap::binary_heap::{Heap, Node};
    use csv::Writer;
    use std::{cell::RefCell, error::Error, rc::Rc};

    #[test]
    fn test_father_of_node_n() {
        // This is the heap used in the subject
        let nodes = Node {
            value: 5,
            cost: 5,
            left_child: Some(Rc::new(RefCell::new(Node {
                value: 21,
                cost: 21,
                left_child: Some(Rc::new(RefCell::new(Node {
                    value: 25,
                    cost: 25,
                    left_child: Some(Rc::new(RefCell::new(Node {
                        value: 80,
                        cost: 80,
                        left_child: None,
                        right_child: None,
                    }))),
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(Node {
                    value: 99,
                    cost: 99,
                    left_child: None,
                    right_child: None,
                }))),
            }))),
            right_child: Some(Rc::new(RefCell::new(Node {
                value: 45,
                cost: 45,

                left_child: Some(Rc::new(RefCell::new(Node {
                    value: 75,
                    cost: 75,

                    left_child: None,
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(Node {
                    value: 51,
                    cost: 51,

                    left_child: None,
                    right_child: None,
                }))),
            }))),
        };
        let heap = Heap {
            size: 8,
            root: Some(Rc::new(RefCell::new(nodes))),
        };
        assert_eq!(heap.path_to_father_of_node(1), None);
        assert_eq!(value!(heap.path_to_father_of_node(2)), 5);
        assert_eq!(value!(heap.path_to_father_of_node(3)), 5);
        assert_eq!(value!(heap.path_to_father_of_node(6)), 45);
        assert_eq!(value!(heap.path_to_father_of_node(8)), 25);
        assert_eq!(value!(heap.path_to_father_of_node(5)), 21);
    }
    #[test]
    fn test_heap_correctness() {
        let number_of_nodes = 50;
        let mut new_heap: Heap<i64> = Heap::default();
        for i in (0..number_of_nodes).rev() {
            new_heap.insert(Rc::new(RefCell::new(Node {
                value: i,
                cost: i as i64,
                left_child: None,
                right_child: None,
            })));
        }

        let mut test_vec: Vec<i64> = Vec::new();
        let min = new_heap.extract_min();
        test_vec.push(min.unwrap());
        for _i in 1..number_of_nodes {
            let min = new_heap.extract_min();
            test_vec.push(min.unwrap());
        }
        assert!(test_vec == (0..number_of_nodes).collect::<Vec<i64>>());
    }

    #[test]
    // too long to run (around 4 minutes on my machine (Acer Swift 3 2021 with an Ryzen 5 4500H running on Manjaro))
    // My values are stored in the csv file
    #[ignore]
    fn test_heap_complexity() -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path("./heap_correctness.csv")?;
        for x in 1..20 {
            let mut new_heap: Heap<i64> = Heap::default();
            let number_of_nodes = 20 * (2 as i64).pow(x);
            let insert_start = std::time::Instant::now();
            for i in (0..number_of_nodes).rev() {
                new_heap.insert(Rc::new(RefCell::new(Node {
                    value: i,
                    cost: i as i64,
                    left_child: None,
                    right_child: None,
                })));
            }

            let insert_duration = insert_start.elapsed();

            let extract_start = std::time::Instant::now();
            for _i in 0..number_of_nodes {
                let _min = new_heap.extract_min();
            }
            let time_to_extract = extract_start.elapsed();
            // println!("{} nodes inserted in {} ms, path to father of last node extracted in {} ms", number_of_nodes, insert_duration.as_micros(), time_to_extract.as_micros());

            wtr.write_record(&[
                number_of_nodes.to_string(),
                insert_duration.as_millis().to_string(),
                time_to_extract.as_millis().to_string(),
            ])?
        }
        wtr.flush()?;
        Ok(())
    }
}
