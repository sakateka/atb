use std::{io, mem};

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let _count: usize = buf.trim().parse().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let numbers = buf.trim().split_whitespace().map(|n| {
        n.parse::<u64>().unwrap()
    }).collect::<Vec<u64>>();

    let mut numbers_iter = numbers.into_iter();
    let mut alpha: u64 = numbers_iter.next().unwrap();
    let mut beta: u64 = numbers_iter.next().unwrap();
    if alpha < beta {
        mem::swap(&mut alpha, &mut beta);
    }
    for n in numbers_iter {
        if n > alpha {
            beta = alpha;
            alpha = n;
        } else {
            if n > beta {
                beta = n;
            }
        }
    }
    println!("{}", alpha * beta);
}
