use std::io;
use std::mem::replace;

pub fn fib_huge_n_modulo_m(n: u64, m: u64) -> u64 {
    let mut f0: u64 = 0;
    let mut f1: u64 = 1;

    let big_m: u64 = m * 10;
    let mut period: u64 = 1;
    let mut found_period: bool = false;

    for i in 0..n {
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


pub fn last_digit_of_sum_of_squares_of_fib(n: u64) -> u8 {
    if n <= 1 {
        return n as u8
    }
    let a = fib_huge_n_modulo_m(n-1, 10);
    let b = fib_huge_n_modulo_m(n, 10);
    (((a+b)*b) % 10) as u8
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: u64 = buf.trim().parse().unwrap();
    println!("{}", last_digit_of_sum_of_squares_of_fib(num));
}
