use std::{io, cmp};

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let count: usize = buf.trim().parse().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let numbers = buf.trim().split_whitespace().map(|n| {
        n.parse::<u64>().unwrap()
    }).collect::<Vec<u64>>();

    let mut result: u64 = 0;
    for i in 0..count {
        for j in (i+1)..count {
            result = cmp::max(result, numbers[i] * numbers[j]);
        }
    }
    println!("{}", result);
}
