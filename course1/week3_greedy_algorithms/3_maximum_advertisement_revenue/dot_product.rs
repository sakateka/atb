use std::io;

fn max_dot_product(mut a: Vec<i64>, mut b: Vec<i64>) -> i64 {
    a.sort();
    b.sort();
    a.iter().zip(b.iter()).fold(0, |acc, i| {
        acc + i.0 * i.1
    })
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let a = buf.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let b = buf.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
    println!("{}", max_dot_product(a, b));
}
