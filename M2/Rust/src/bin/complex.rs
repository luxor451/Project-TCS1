pub struct Complex {
    // complete this definition
}

impl Complex {
    // implement (at least) these associated functions

    fn new(r: f64, i: f64) -> Self {
        panic!("not yet implemented !")
    }
    fn add(&self, other: &Self) -> Self {
       panic!("not yet implemented !")
    }
    fn sub(&self, other: &Self) -> Self {
        panic!("not yet implemented !")
    }
    fn mult(&self, other: &Self) -> Self {
       panic!("not yet implemented !")
    }
    // multiplication by a scalar
    fn scal(&self, scal: f64) -> Self {
        panic!("not yet implemented !")
    }
    fn module(&self) -> f64 {
        panic!("not yet implemented !")
    }
    fn argument(&self) -> f64 {
        panic!("not yet implemented !")
    }
}

pub struct Rectangle {
    // complete this definition
}

impl Rectangle {
    // implement (at least) these associated functions

    // the width of the self rectangle
    fn width(&self) -> f64 {
        panic!("not yet implemented !")
    }
    // the height of the self rectangle
    fn height(&self) -> f64 {
        panic!("not yet implemented !")
    }
    // pick up some complex point inside the self rectangle
    fn pick(&self) -> Complex {
        panic!("not yet implemented !")
    }
    // split the self rectangle along its largest dimension
    // iff it is larger than eps, the required precision
    fn split(&self, eps: f64) -> Option<(Self, Self)> {
        panic!("not yet implemented !")
    }
}

// my_poly(z) == (z^3 - 1)
fn my_poly(z: &Complex) -> Complex {
    let r = &Complex::new(1., 0.);
    z.mult(z).mult(z).sub(r)
}

// compute the number of times the argument (of my_poly) crosses 0
// along the line z1--z2
// proceed by iteratively sampling my_poly along z1--z2 with steps
// smaller than h
fn zero_crossings_line(z1: &Complex, z2: &Complex, h: f64) -> i64 {
    let delta_z = z2.sub(z1);
    // n is the number of (small enough) steps
    let n = f64::ceil(delta_z.scal(1. / h).module());
    // arg is the current argument of my_poly(z) along z1--z2
    let mut arg = my_poly(z1).argument();
    // pos records the sign of the current argument
    let mut pos: bool = arg > 0.;
    // zeroes records the number of times arg has crossed 0
    let mut zeroes: i64 = 0;
    for i in 1..=(n as i64) {
        let c1 = n - (i as f64);
        let c2 = i as f64;
        let zc1 = z1.scal(c1 / n);
        let zc2 = z2.scal(c2 / n);
	// z is the ith point along z1--z2
        let z = zc1.add(&zc2);
	// complete the loop body !!
	panic!("not yet implemented !")
    }
    return zeroes;
}
// compute the topological degree (of my_poly) along the closed curve
// defined by the rect rectangle
fn topological_degree_rectangle(rect: &Rectangle, h: f64) -> i64 {
    panic!("not yet implemented !")
}
// look for a root of my_poly within the rect rectangle
// optionally return a small enough rectangle where a root lies
fn find_root(rect: Rectangle, h: f64) -> Option<Rectangle> {
    panic!("not yet implemented !")
}
// main application
// look for a root of my_poly within a given rectangle
// and display a complex root upto a given precision,
// or the absence of such a root 
fn main() {
    panic!("not yet implemented !")
}
