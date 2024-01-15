/* use std::io; */


fn fact(n:i32) -> i32{
    let mut res:i32 = 1;
    for _i in 1..n {
        res *= _i;
    }
    return res;
}

fn main(){
    let x:i32  = fact(13);
    println!("{}", x);
}