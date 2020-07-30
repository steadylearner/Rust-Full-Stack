#![feature(test)]
extern crate test;
use test::Bencher;
use test::black_box;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[bench]
fn bench_fib(b: &mut Bencher) {
    b.iter(|| fibonacci(black_box(20)));
}


