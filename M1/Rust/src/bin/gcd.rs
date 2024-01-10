use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a;
    let mut y = b;

    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }

    return x;
}

fn main() {
    let i1: i32 = 42;
    let i2: i32 = 1024;

    println!("GCD of {} and {} is {}",
             i1, i2, gcd(i1, i2));
}
