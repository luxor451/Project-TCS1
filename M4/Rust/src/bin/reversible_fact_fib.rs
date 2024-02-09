/* fn fact(n: u64) -> u64 {
    let mut fact_n = 1;
    for i in 2..=n {
        fact_n *= i;
    }
    return fact_n;
}

fn fib(n: u64) -> (u64, u64) {
    let mut fib_n = (0, 1);
    for _ in 1..n {
        fib_n = (fib_n.1, fib_n.0 + fib_n.1)
    }
    return fib_n;
}

fn fib_rec(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib_rec(n - 2) + fib_rec(n - 1)
    }
}
 */

pub mod exo1 {
    #[derive(Debug)]
    pub enum Result {

        Ok, 
        Undef,
        ArithError,
    }
    pub fn fact(n : u64, fact_n : &mut u64) -> Result {
        *fact_n = 1;
        for i in 2..=n {
            if let Some(res) = u64::checked_mul(*fact_n, i){
                *fact_n = res;
            }
            else {
                return Result::ArithError;
            }
        }
        return Result::Ok
    }


    

    
    pub fn test_fact(){
        let mut fact_n:u64 = 0;
        let res = fact(5, &mut fact_n);
        println!("fact(5) = {}, return {:?}", fact_n, res);
    }
   
}




pub mod exo2{
    #[derive(Debug)]
    pub enum Result {

        Ok, 
        Undef,
        ArithError,
    }
    pub fn inv_fact(n : &mut u64, fact_n : u64) -> Result{
        let mut res : u64 = 1;
        let mut courant : f64 = fact_n as f64;
        while courant > 1. {
            res += 1;
            courant /= res as f64;
            
        }

        if courant == 1. {
            *n = res;
            return Result::Ok
        }
        else {
            return Result::ArithError;
        }
        
    }

    pub fn test_inv_fact(){
        let mut n:u64 = 0;
        let res = inv_fact(&mut n, 120);
        println!("inv_fact(120) = {}, return {:?}", n, res);    
    }
    
/*     pub fn inv_fib (n : &mut u64, f_n : u64){

    } */
}
fn main() {
    //exo2::test_fib();
    exo2::test_inv_fact()
}
