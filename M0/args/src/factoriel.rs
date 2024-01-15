use std::env;


fn factoriel (n: u64) -> u64{
    if n == 1 || n == 0 {
        return n;
    }
    else {
        return n * factoriel(n-1)
    }
}


fn main (){
    println("{}", factoriel(10))
}