use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fibonacci::fibonacci_sequence;
use fibonacci::sort_arr;

fn sort_arr_benchmark(c: &mut Criterion) {
    let mut arr = black_box([6, 2, 4, 1, 9, -2, 5]);
    c.bench_function("sorting algorithm", |b| b.iter(|| sort_arr(&mut arr)));
}

fn fibonacci_sequence_benchmark(c: &mut Criterion) {
    let num: usize = 40;
    c.bench_function("fibonacci algorithm", |b| {
        b.iter(|| fibonacci_sequence(num))
    });
}

// criterion_group!(benches, sort_arr_benchmark, fibonacci_sequence_benchmark);
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = sort_arr_benchmark, fibonacci_sequence_benchmark
}

criterion_main!(benches);

/*
Since we've disabled the default benchmark harness, we need to add our own:
    #[macro_use]
    extern crate criterion;
    use criterion::Criterion;
    fn bench_method1(c: &mut Criterion) {}
    fn bench_method2(c: &mut Criterion) {}
    criterion_group!(benches, bench_method1, bench_method2);
    criterion_main!(benches);
    The criterion_main macro expands to a main function which runs all of the benchmarks in the given groups.
*/
