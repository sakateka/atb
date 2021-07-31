use std::io;
use std::mem::replace;

pub fn fib(n: usize) -> u64 {
    let mut f0: u64 = 0;
    let mut f1: u64 = 1;
    for _ in 0..n {
        let f2 = f0 + f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}

#[allow(dead_code)]
fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: usize = buf.trim().parse().unwrap();
    println!("{}", fib(num));
}
