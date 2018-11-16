use std::io;

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.trim().split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    println!("{}", a + b);
}
