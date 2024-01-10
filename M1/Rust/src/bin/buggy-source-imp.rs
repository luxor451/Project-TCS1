/* This simple program should compute the square root of 2 using
 * Newton's method with a precision of 1E-6. */

fn main() {
    let max_number_of_iterations: i32 = 20;
    let epsilon: f64                  = 1E-6;
    let solution: bool                = false;
    let mut x0: f64                   = 2.0

    for i in 1..max_number_of iterations {
        let y: f64      = x0 * x0 - 2.0;
        let yprime: f64 = 2 * x0;
        let x1: f64     = x0 -y / yprime;

        if (f64::abs(x1 - x0) <= epsilon * f64::abs(x1)) {
            solution = true;
            break;
        }

        x0 = x1;
    }

    if solution {
        println!("sqrt(2) = ", x1);
    } else {
        println!("no convergence in {} iterations...\n", max_number_of_iterations);
    }
}
