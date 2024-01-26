fn main() -> () {
    let eqn_real = QuadraticEquation {
        a: 2.0,
        b: 9.0,
        c: -5.0,
    };
    let eqn_one = QuadraticEquation {
        a: 4.0,
        b: 4.0,
        c: 1.0,
    };
    let eqn_compl = QuadraticEquation {
        a: 3.0,
        b: 5.0,
        c: 7.0,
    };

    print_solutions_quadratic_equation(&eqn_real);
    println!();

    print_solutions_quadratic_equation(&eqn_one);
    println!();

    print_solutions_quadratic_equation(&eqn_compl);
    println!();
}
