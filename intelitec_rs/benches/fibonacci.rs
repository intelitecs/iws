
extern crate criterion;
extern crate intelitec_rs;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use intelitec_rs::{slow_fibonacci, fast_fibonacci};


fn slow_fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("slow fibonacci 8", |bencher| bencher.iter(|| slow_fibonacci(black_box(8))));
}


fn fast_fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fast fibonacci 8", |bencher| bencher.iter(|| fast_fibonacci(black_box(8))));
}




criterion_group!(fib_bench, slow_fibonacci_benchmark, fast_fibonacci_benchmark);
//criterion_group!(fast_fib_bench, fast_fibonacci_benchmark);
criterion_main!(fib_bench);
