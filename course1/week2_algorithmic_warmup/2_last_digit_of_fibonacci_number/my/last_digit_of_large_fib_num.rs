use std::io;
use std::mem::replace;

pub fn last_digit_fib(n: usize) -> u8 {
    let mut f0: u8 = 0;
    let mut f1: u8 = 1;
    for _ in 0..n {
        let f2 = (f0 + f1) % 10;
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: usize = buf.trim().parse().unwrap();
    println!("{}", last_digit_fib(num));
}
