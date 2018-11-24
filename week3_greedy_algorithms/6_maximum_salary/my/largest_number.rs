use std::io;

fn largest_number(mut parts: Vec<&str>) -> Vec<&str> {
    parts.sort_by(|a , b| {
        let num1: u32 = [*a, *b].concat().parse().unwrap();
        let num2: u32 = [*b, *a].concat().parse().unwrap();
        // reverse order
        num2.cmp(&num1)
    });
    parts
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let numbers = buf.trim().split_whitespace().collect::<Vec<&str>>();
    println!("{}", largest_number(numbers).concat());
}
