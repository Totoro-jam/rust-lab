//! Criterion benchmarks for add and fibonacci.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustlab07::{add, fibonacci};

fn bench_add(c: &mut Criterion) {
    c.bench_function("add", |b| b.iter(|| add(black_box(42), black_box(58))));
}

fn bench_fibonacci_20(c: &mut Criterion) {
    c.bench_function("fibonacci(20)", |b| b.iter(|| fibonacci(black_box(20))));
}

fn bench_fibonacci_40(c: &mut Criterion) {
    c.bench_function("fibonacci(40)", |b| b.iter(|| fibonacci(black_box(40))));
}

criterion_group!(benches, bench_add, bench_fibonacci_20, bench_fibonacci_40);
criterion_main!(benches);
