/* use std::io; */


fn compute_pi(n:i32) -> f64{
    let mut res:f64 = 0 as f64;
    for _i in 0..n+1 {
        if _i % 2 == 0 {
            res = res + 1.0/(2.0*(_i as f64) + 1.0);
        } else {
            res = res - 1.0/(2.0*(_i as f64) + 1.0);
        }
    }
    return 4 as f64 * res;
}

fn main(){
    let n:i32 = 1000000;
    let x:f64 = compute_pi(n);
    println!("{:.64}", x);
}