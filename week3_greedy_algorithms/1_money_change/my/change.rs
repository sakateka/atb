use std::io;

pub fn fib(n: u32) -> u32 {
    let mut reminder = n;
    let mut coins: u32 = 0;
    if reminder >= 10 {
        coins += reminder / 10;
        reminder = reminder % 10;
    }
    if reminder >= 5 {
        coins += reminder / 5;
        reminder = reminder % 5;
    }
    coins += reminder;
    coins
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: u32 = buf.trim().parse().unwrap();
    println!("{}", fib(num));
}
