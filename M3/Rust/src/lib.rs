use core::f64::consts::PI;

pub enum Complex {
    Car { real: f64, imag: f64 },
    Pol { modu: f64, argu: f64 },
}

pub fn real(x: &Complex) -> f64 {
    match x {
        Complex::Car { real, .. } => *real,
        Complex::Pol { modu, argu } => modu * f64::cos(*argu),
    }
}

pub fn imag(x: &Complex) -> f64 {
    match x {
        Complex::Car { real: _, imag } => *imag,
        Complex::Pol { modu, argu } => modu * f64::sin(*argu),
    }
}

pub fn modu(x: &Complex) -> f64 {
    match x {
        Complex::Car { real, imag } => f64::sqrt(f64::powf(*real, 2.0) + f64::powf(*imag, 2.0)),
        Complex::Pol { modu, .. } => *modu,
    }
}

pub fn argu(x: &Complex) -> Option<f64> {
    match x {
        Complex::Car { real, imag } => {
            if *real == 0.0 && *imag == 0.0 {
                None
            } else {
                Some(imag.atan2(*real))
            }
        }
        Complex::Pol { modu: _, argu } => Some(*argu),
    }
}

pub fn conjugate(x: &Complex) -> Complex {
    match x {
        Complex::Car { real, imag } => Complex::Car {
            real: *real,
            imag: -*imag,
        },
        Complex::Pol { modu, argu } => Complex::Pol {
            modu: *modu,
            argu: -*argu,
        },
    }
}

pub fn mult(x: &Complex, y: &Complex) -> Complex {
    match (x, y) {
        (
            Complex::Car {
                real: real_x,
                imag: imag_x,
            },
            Complex::Car {
                real: real_y,
                imag: imag_y,
            },
        ) => Complex::Car {
            real: *real_x * *real_y - *imag_x * *imag_y,
            imag: *real_x * *imag_y + *imag_x * *real_y,
        },
        (
            Complex::Pol {
                modu: modu_x,
                argu: argu_x,
            },
            Complex::Pol {
                modu: modu_y,
                argu: argu_y,
            },
        ) => Complex::Pol {
            modu: *modu_x * *modu_y,
            argu: (*argu_x + *argu_y) % (2.0 * PI),
        },
        (Complex::Car { .. }, Complex::Pol { .. }) => mult(
            &x,
            &Complex::Car {
                real: real(y),
                imag: imag(y),
            },
        ),
        (Complex::Pol { .. }, Complex::Car { .. }) => mult(&y, &x),
    }
}

pub fn sqr_norm(x: &Complex) -> f64 {
    real(&mult(&x, &conjugate(&x)))
}

pub struct QuadraticEquation {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub enum QuadraticEquationSolution {
    DoubleRealSolutions { sol_1: f64, sol_2: f64 },
    OneRealSolution { sol: f64 },
    ComplexSolutions { sol_1: Complex, sol_2: Complex },
}

pub fn solve_quadratic_equation(eq: &QuadraticEquation) -> QuadraticEquationSolution {
    let delta = eq.b * eq.b - 4.0 * eq.a * eq.c;
    let den = 2.0 * eq.a;
    if delta > 0.0 {
        let sqrt_delta = f64::sqrt(delta);

        QuadraticEquationSolution::DoubleRealSolutions {
            sol_1: (-eq.b - sqrt_delta) / den,
            sol_2: (-eq.b + sqrt_delta) / den,
        }
    } else if delta < 0.0 {
        let sqrt_delta = f64::sqrt(-delta);

        QuadraticEquationSolution::ComplexSolutions {
            sol_1: Complex::Car {
                real: -eq.b / den,
                imag: -sqrt_delta / den,
            },

            sol_2: Complex::Car {
                real: -eq.b / den,
                imag: sqrt_delta / den,
            },
        }
    } else {
        QuadraticEquationSolution::OneRealSolution { sol: -eq.b / den }
    }
}

pub fn print_solutions_quadratic_equation(eq: &QuadraticEquation) {
    match solve_quadratic_equation(&eq) {
        QuadraticEquationSolution::DoubleRealSolutions { sol_1, sol_2 } => {
            println!(
                "There are 2 real solutions for {}*x² + {}*x + {} = 0:",
                eq.a, eq.b, eq.c
            );
            println!("  - {}", sol_1);
            println!("  - {}", sol_2);
        }
        QuadraticEquationSolution::ComplexSolutions { sol_1, sol_2 } => {
            println!(
                "There are 2 complex solutions for {}*x² + {}*x + {} = 0:",
                eq.a, eq.b, eq.c
            );
            println!("  - {} + {} * i", real(&sol_1), imag(&sol_2));
            println!("  - {} + {} * i", real(&sol_1), imag(&sol_2));
        }
        QuadraticEquationSolution::OneRealSolution { sol } => {
            println!(
                "There is one real solution for {}*x² + {}*x + {} = 0: {}",
                eq.a, eq.b, eq.c, sol
            );
        }
    }
}

#[cfg(test)]
pub mod tests {
    use core::f64::consts::PI;

    #[test]
    fn check_conjugate() {
        let cc1 = crate::Complex::Car {
            real: -2.0,
            imag: 1.0,
        };
        let cc2 = crate::conjugate(&cc1);
        let cp1 = crate::Complex::Pol {
            modu: 1.0,
            argu: (PI / 4.0),
        };
        let cp2 = crate::conjugate(&cp1);

        assert_eq!(crate::real(&cc1), crate::real(&cc2));
        assert_eq!(crate::imag(&cc1), -1.0 * crate::imag(&cc2));
        assert_eq!(crate::modu(&cp1), crate::modu(&cp2));
        assert_eq!(
            crate::argu(&cp1).expect("cannot happen!"),
            -1.0 * (crate::argu(&cp2)).expect("cannot happen!")
        );
    }
}
