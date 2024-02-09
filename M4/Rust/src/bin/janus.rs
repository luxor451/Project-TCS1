// type Mod describes allowed (i.e. reversible) modifiers
// it is a simple data that may be copied and cloned at will
#[derive(Copy, Clone)]
enum Mod {
    Add,
    Sub,
    Mul,
    Div,
    Xor,
}
// evaluation of: v1 op v2, where op is a modifier
// depends on boolean b, accounting for forward or
// backward computation
// Add and Sub are inverse of each other
// Mul and Div are inverse of each other
// only when there is no remainder
// Xor is its own inverse
impl Mod {
    fn eval(self, v1: i64, v2: i64, b: bool) -> i64 {
        match self {
            Mod::Add => if b {v1+v2} else {v1-v2},
            Mod::Sub => if b {v1-v2} else {v1+v2},
            Mod::Mul => if b {v1*v2} else {assert_eq!(v1%v2, 0); v1/v2},
            Mod::Div => if b {assert_eq!(v1%v2, 0); v1/v2} else {v1*v2},
            Mod::Xor => if v1 != v2 {1} else {0},
        }
    }
}

use std::{collections::HashMap, intrinsics::needs_drop};

// variables are static constant strings
type Var = &'static str;
// an environment binds variables to integer values
type Env = HashMap<Var, i64>;

// impl Fn(&mut Env, bool) is the "type" of closures
// taking a mutable environment and a boolean forward/backward mode.
// such a closure represents a Janus program and therefore
// applying a closure may modify the environment passed as argument

// impl Fn(& Env) -> i64 is the "type" of closures
// taking an immutable environment and returning an integer.
// such a closure represents a pure expression that does not
// mutate its environment when evaluated.

// the minimal closure/program that does nothing.
fn null() -> impl Fn(&mut Env, bool) {
    return |_, _| {};
}

// the sequence operator that takes 2 closures/programs p1 and p2
// and returns a closure/program.
// Evaluation order depends on forward/backward mode b.
fn seq(p1: impl Fn(&mut Env, bool), p2: impl Fn(&mut Env, bool)) -> impl Fn(&mut Env, bool) {
    return move |env, b| {
        if b {
            p1(env, b);
            p2(env, b);
        } else {
            p2(env, b);
            p1(env, b);
        }
    }
}

// atomic instruction that exchanges the values of two variables
// swap is its own inverse, so b is irrelevant
fn swap(x1: Var, x2: Var) -> impl Fn(&mut Env, bool) {
    return move |env, _| {
        let v1 = *env.get(x1).unwrap();
        let v2 = *env.get(x2).unwrap();
        let rv1 = env.get_mut(x1).unwrap();
        *rv1 = v2;
        let rv2 = env.get_mut(x2).unwrap();
        *rv2 = v1;
    }
}

// atomic instruction that updates the value of a variable
// through an allowed modifier and an closure/expression to be evaluated
fn assign(x: Var, op: Mod, e: impl Fn(&Env) -> i64) -> impl Fn(&mut Env, bool) {
    return |env, b| panic!("not implemented !");
}

// reversible conditional statement.
// The condition/assertion status of expressions pre and post
// depends on forward/backward mode b
// pre and post must be both 1 or 0 during an execution
// // conditional reversible statement.
// The condition/assertion status of expressions pre and post
// depends on forward/backward mode b
//
//  condition/assertion   |--> p_then --|    assertion/condition
//               pre  ----|             |---->  post
//                        |--> p_else --|
fn conditional(
    pre: impl Fn(&Env) -> i64,
    p_then: impl Fn(&mut Env, bool),
    p_else: impl Fn(&mut Env, bool),
    post: impl Fn(&Env) -> i64,
) -> impl Fn(&mut Env, bool) {
    return |env, b| panic!("not implemented !");
}

// reversible loop statement.
// The condition/assertion status of expressions pre and post
// depends on forward/backward mode b
// pre must hold true only once, at first iteration
// post holds true only once, at last iteration
//
//              |-------------> p_forward ------------>|
//     assertion/condition                       condition/assertion
// -->          pre                                    post           -->
//              |<------------- p_backward <-----------|
fn repeat(
    pre: impl Fn(&Env) -> i64,
    p_forward: impl Fn(&mut Env, bool),
    p_backward: impl Fn(&mut Env, bool),
    post: impl Fn(&Env) -> i64,
) -> impl Fn(&mut Env, bool) {
    return |env, b| panic!("not implemented !");
}
// A fibonacci procedure in Janus
// with initial parameters x1, x2 and n.
//x1 += 1
//x2 += 1
//from x1 = x2
//do
//    x1 += x2
//    x1 <=> x2
//    n -= 1
//until n = 0
fn fibonacci(x1: Var, x2: Var, n: Var) -> impl Fn(&mut Env, bool) {
    return seq(
        assign(x1, Mod::Add, |_| 1),
        seq(
            assign(x2, Mod::Add, |_| 1),
            repeat(
                move |env| {
                    if *(env.get(&x1).unwrap()) == *(env.get(&x2).unwrap()) {
                        1
                    } else {
                        0
                    }
                },
                seq(
                    assign(x1, Mod::Add, move |env| *(env.get(x2).unwrap())),
                    seq(swap(x1, x2), assign(n, Mod::Sub, |_| 1)),
                ),
                null(),
                move |env| if *(env.get(&n).unwrap()) == 0 { 1 } else { 0 },
            ),
        ),
    );
}

// testing the Fibonacci procedure in forward mode
pub fn test_fibonacci() {
    let x1 = "x1";
    let x2 = "x2";
    let n = "n";
    let mut env: Env = HashMap::new();
    env.insert(x1, 0);
    env.insert(x2, 0);
    env.insert(n, 4);
    let prog = fibonacci(x1, x2, n);
    prog(&mut env, true);
    println!(
        "{} = {}, {} = {}, {} = {}",
        x1,
        env.get(x1).unwrap(),
        x2,
        env.get(x2).unwrap(),
        n,
        env.get(n).unwrap()
    );
}

// testing the Fibonacci procedure in backward mode
pub fn test_reverse_fibonacci() {
    let x1 = "x1";
    let x2 = "x2";
    let n = "n";
    let mut env: Env = HashMap::new();
    env.insert(x1, 5);
    env.insert(x2, 8);
    env.insert(n, 0);
    let prog = fibonacci(x1, x2, n);
    prog(&mut env, false);
    println!(
        "{} = {}, {} = {}, {} = {}",
        x1,
        env.get(x1).unwrap(),
        x2,
        env.get(x2).unwrap(),
        n,
        env.get(n).unwrap()
    );
}

fn main() {
    test_fibonacci();
    test_reverse_fibonacci()
}
