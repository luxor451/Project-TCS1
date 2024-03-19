use std::fmt::{Display, Formatter, Result};

pub trait PriorityQueue<K: Ord, V> {
    fn empty() -> Self;
    fn insert(&mut self, key: K, value: V);
    fn merge(&mut self, other: &mut Self);
    fn get_min(&self) -> Option<(&K, &V)>;
    fn extract_min(&mut self) -> Option<(K, V)>;
}
// Mise en place de la structure de données de file de priorité de type Leftist Binary Tree
// Structures construites de manière similaire à celles de la liste chaînée de la semaine 8
struct Node<K: Ord, V> {
    height: usize,
    key: K,
    value: V,
    left: LBTree<K,V>,
    right: LBTree<K, V>,
}

pub struct LBTree<K: Ord, V> {
    link: Option<Box<Node<K, V>>>
}

impl <K: Ord, V> Node<K, V> {
    // Fonction qui permet de s'assurer de la validité de la structure de l'arbre
    fn normalize(&mut self){
        let height = usize::max(self.left.height(),self.right.height()) + 1;
        if self.left.height() < self.right.height() {
            std::mem::swap(&mut self.left, &mut self.right);
        }
        self.height = height;
    }
    
}

impl<K: Ord + Display, V: Display> Display for LBTree<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.link {
            Option::Some(node) => {
                                    write!(f, "left: {}", node.left)?; // Appelle récursivement fmt par definition de write!()
                                    write!(f, "({}, {}) : {}", node.key, node.value, node.height)?;
                                    write!(f, "right: {}", node.right)?;
                                    Ok(())
                                    },
            Option::None =>         write!(f, "()"), // Pas de "?" fait retourner le Result de write!(), normalement Ok(())
        }
    }
}

impl<K: Ord, V> LBTree<K, V> {
    fn height(&self) -> usize {
        self.link.as_ref().map_or(0, |node| node.height)
    }
    // here write auxiliary functions
    fn new_node(key: K, value: V, left: Self, right: Self) -> Self {
        let mut node: Node<K, V> = Node {
            height: 0,
            key,
            value,
            left,
            right,
        };
        node.normalize();
        Self {link: Some(Box::new(node))}
    }
}
impl<K: Ord, V> PriorityQueue<K, V> for LBTree<K, V> {
    fn empty() -> Self {
        Self { link: None }
    }
    fn merge(&mut self, other: &mut Self) {
        if let Some(mut other_node) = other.link.take() {
            if let Some(node) = self.link.as_mut() {
                if node.key <= other_node.key {
                    node.right.merge(other);
                    node.normalize();
                } else {
                    std::mem::swap(node, &mut other_node);
                    self.merge(other);
                }
            } else {
                std::mem::swap(self, other);
            }
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.merge(&mut Self::new_node(key, value, Self::empty(), Self::empty()));
    }
    fn get_min(&self) -> Option<(&K, &V)> {
        self.link.as_ref().map(|node| (&node.key, &node.value))
    }
    fn extract_min(&mut self) -> Option<(K, V)> {
        self.link.take().map(|mut node| 
            {
            node.left.merge(&mut node.right);
            self.link = node.left.link;
            (node.key, node.value)
            })
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get_min() {
        let mut pq: LBTree<i32, &str> = LBTree::empty();
        pq.insert(3, "three");
        pq.insert(1, "one");
        pq.insert(2, "two");
        println!("display : {}", pq);
        assert_eq!(pq.get_min(), Some((&1, &"one")));
    }

    #[test]
    fn test_extract_min() {
        let mut pq: LBTree<i32, &str> = LBTree::empty();
        pq.insert(3, "three");
        pq.insert(1, "one");
        pq.insert(2, "two");

        assert_eq!(pq.extract_min(), Some((1, "one")));
        assert_eq!(pq.get_min(), Some((&2, &"two")));
    }

    #[test]
    fn test_merge() {
        let mut pq1: LBTree<i32, &str> = LBTree::empty();
        pq1.insert(3, "three");
        pq1.insert(1, "one");

        let mut pq2: LBTree<i32, &str> = LBTree::empty();
        pq2.insert(2, "two");
        pq2.insert(4, "four");

        pq1.merge(&mut pq2);

        assert_eq!(pq1.get_min(), Some((&1, &"one")));
        assert_eq!(pq2.get_min(), Some((&2, &"two")));
    }
}