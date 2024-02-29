//! A demo for unit testing and integration testing (see ../test directory)
//!

pub fn fact(n: i32) -> i32 {
    let mut fact: i32 = 1;
    for i in 1..n + 1 {
        fact = i * fact;
    }
    return fact;
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fact() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(4), 24);
    }
}
