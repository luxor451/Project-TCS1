use leftist::*;

fn sort<K: Ord + Copy, Q: PriorityQueue<K, ()>>(v: &mut Vec<K>) {
    panic!("not implemented !");
}

fn main() {
    let mut v = vec![6, 3, 4, 1, 2, 3, 0, 2];
    sort::<i32, LBTree<i32, ()>>(&mut v);
    for e in v {
        println!("{}", e);
    }
}
