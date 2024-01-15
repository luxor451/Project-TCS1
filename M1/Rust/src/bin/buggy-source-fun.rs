fn incr(i: i32) -> i32{
    return i + 1;
}

fn pretty_print(i: i32) {
    println!("Here is {}", i);
}

fn add(i: f64, j: f64) {
    println!("{}", i + j);
}

fn main() {
    let i: i32 = 1;
    let j: f64 = 3.5;

    pretty_print(i);

    println!("inc({}) = {}", i, incr(i));

    println!("Now, an addition!");

    add(i as f64, j);
}
