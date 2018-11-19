use std::io;
use std::mem::replace;

pub fn fib_huge_n_modulo_m(n: u64, m: u64) -> u64 {
    let mut f0: u64 = 0;
    let mut f1: u64 = 1;

    let big_m: u64 = m * 10;
    let mut period: u64 = 1;
    let mut found_period: bool = false;

    for i in 0..(n*1000) {
        let f2 = f0 + f1 % big_m;
        f0 = replace(&mut f1, f2);

        if i > 1 && f0 % m == 0 {
            found_period = true;
        } else {
            if found_period {
                if f0 % m == 1 {
                    period -= 1;
                    break;
                } else {
                    found_period = false;
                }
            }
        }
        period += 1;
    }

    // shortcut
    if !found_period {
        return f0 % m;
    }

    let small_n = n % period;

    f0 = 0;
    f1 = 1;
    for _ in 0..small_n {
        let f2 = f0 + f1 % big_m;
        f0 = replace(&mut f1, f2);
    }
    f0 % m
}

fn main() {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.trim().split_whitespace();
    let n: u64 = iter.next().unwrap().parse().unwrap();
    let m: u64 = iter.next().unwrap().parse().unwrap();
    println!("{}", fib_huge_n_modulo_m(n, m));
}
