use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use std::mem::replace;

// from https://docs.rs/num-bigint/0.2.1/num_bigint/
pub fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

pub fn fib_internal(n: BigUint) -> BigUint {
    let one: BigUint = One::one();
    if n <= one {
        return n;
    }
    let a = n.clone() - one.clone();
    fib_internal(a.clone()) + fib_internal(a - one)
}

pub fn fib_naive(n: usize) -> BigUint {
    fib_internal(n.to_biguint().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_fib(b: &mut Bencher) {
        b.iter(|| fib(10));
    }
    #[bench]
    fn bench_fib_naive(b: &mut Bencher) {
        b.iter(|| fib_naive(10));
    }
}
