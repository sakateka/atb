use std::io;
use std::mem::replace;

pub fn last_digit_fib_sum(n: u64) -> u8 {
    if n <= 1 {
        return n as u8
    }

    let mut f0: u8 = 0;
    let mut f1: u8 = 1;
    let mut fsum_mod: u8 = 1;

    let mut period: u64 = 1;
    let mut found_period: bool = false;

    for i in 0..(n-1) {
        let f2 = (f0 + f1) % 10;
        f0 = replace(&mut f1, f2);
        fsum_mod = (fsum_mod + f2) % 10;

        if i > 1 && fsum_mod == 0 {
            found_period = true;
        } else {
            if found_period {
                if fsum_mod == 1 {
                    break;
                }
                found_period = false;
            }
        }
        period += 1;
    }
    let small_n = (n-1) % period;
    // shortcut
    if !found_period || small_n == 0 {
        return fsum_mod;
    }


    let mut f0: u8 = 0;
    let mut f1: u8 = 1;
    let mut fsum_mod: u8 = 1;
    for _ in 0..small_n {
        let f2 = (f0 + f1) % 10;
        f0 = replace(&mut f1, f2);
        fsum_mod = (fsum_mod + f2) % 10;
    }
    fsum_mod
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: u64 = buf.trim().parse().unwrap();
    println!("{}", last_digit_fib_sum(num));
}
