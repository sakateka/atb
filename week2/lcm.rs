use std::io;
use std::mem;

fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        a %= b;
        mem::swap(&mut a, &mut b);
    }
    a
}


fn lcm(a: u32, b: u32) -> u64 {
    let mul: u64 = a as u64 * b as u64;
    mul / gcd(a, b) as u64
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.trim().split_whitespace();
    let a: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();

    println!("{}", lcm(a, b));
}
