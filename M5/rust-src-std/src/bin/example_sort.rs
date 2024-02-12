use sort::*;

fn main() {
    let mut arr: [i64; 10] = [5, 4, -1, 0, 3, 4, 7, 2, 1, 9];
    let mut arr2: [i64; 10] = [5, 4, -1, 0, 3, 4, 7, 2, 1, 9];
    let mut aux: [i64; 10] = arr.clone();
    merge::sort(&mut arr, &mut aux);

    println!("{:?}", arr);

    selection::sort(&mut arr2);

    println!("{:?}", arr2);
}
