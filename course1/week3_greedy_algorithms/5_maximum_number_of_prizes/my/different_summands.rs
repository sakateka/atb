use std::io;

pub fn different_summands(n: usize) -> Vec<usize> {
    let mut summands = Vec::new();
    let mut remain = n;
    let mut current = 0;
    while remain > current {
        current += 1;
        summands.push(current);
        remain -= current;
    }
    let last = summands.len() - 1;
    summands[last] += remain;
    summands
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: usize = buf.trim().parse().unwrap();
    let summands = different_summands(num);
    println!("{}", summands.len());
    println!("{}", summands.iter().fold(String::new(), |acc, num|{
        let sep = if acc.len() > 0 { " " } else { "" };
        acc + sep + &num.to_string()
    }));
}
