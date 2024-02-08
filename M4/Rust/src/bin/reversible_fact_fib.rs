fn fact(n: u64) -> u64 {
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


pub mod exo1 {
    #[derive(Debug)]
    enum Result {
        Ok, 
        Undef,
        ArithError,
    }
    pub fn fact(n : u64, fact_n : &mut u64) -> Result {
        *fact_n = 1;
        for i in 2..n {
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

fn main() {
    //exo2::test_fib();
    //exo2::test_fact()
}
