fn fact(n: u64) -> u64 {
    let mut fact_n = 0;
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

fn main() {
    //exo2::test_fib();
    //exo2::test_fact()
}
