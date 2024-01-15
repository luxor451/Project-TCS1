fn factoriel (n: u64) -> u64{
    if n == 1 || n == 0 {
        return n;
    }
    else {
        return n * factoriel(n-1)
    }
}

fn inc(mut i : i64) {
    i = i + 1;
    }

fn swap(mut x: i64, mut y: i64) {
    let temp : i64 = x;
    x = y;
    y = temp;
    }

fn main (){
    let ys: [i32; 500] = [100; 500];
    println!("{}", ys[100]);
    
}