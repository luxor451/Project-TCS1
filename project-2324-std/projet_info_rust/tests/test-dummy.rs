fn test_dummy_addition() {
    print!("  | tests for addition... ");
    assert_eq!(1 + 1, 2);
    assert_eq!(1 + 2, 3);
    println!("OK!");
}

fn test_dummy_multiplication() {
    print!("  | tests for multiplication... ");
    assert_eq!(1 * 1, 1);
    assert_eq!(2 * 2, 4);
    println!("OK!");
}

fn main() {
    println!("* Starting dummy tests...");

    test_dummy_addition();
    test_dummy_multiplication();

    println!("  +-> OK!");
}