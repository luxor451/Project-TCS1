use core::f64::consts::PI;

fn main() -> () {
    let cc1 = Complex::Car {
        real: -2.0,
        imag: 1.0,
    };

    let cc2 = Complex::Pol {
        modu: 1.0,
        argu: PI / 4.0,
    };

    let res = mult(&cc1, &cc2);

    println!(
        "({} + {} * i) * ({} + {} * i) = {} + {} * i",
        real(&cc1),
        imag(&cc1),
        real(&cc2),
        imag(&cc2),
        real(&res),
        imag(&res)
    );
}
