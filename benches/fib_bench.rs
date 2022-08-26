extern crate test;

#[cfg(test)]
mod tests {

    use crate::math::{fib, fib_naive};
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
