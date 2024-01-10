fn inc(i: i32) {
    return i + 1;
}

fn pretty_print(i: i32) {
    println!("Here is {}", i);
}

fn add(i: i32, j: i32) {
    println!("{}", i + j);
}

fn main() {
    let i: i32 = 1;
    let j: f64 = 3.5;

    pretty_print(i);

    println!("inc({}) = {}", i, incr(i));

    pretty_print("Now, an addition!");

    add(i, j);
}
