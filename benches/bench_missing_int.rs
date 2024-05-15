use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use iterextd::IterExtd;

fn missing_integer_uqsort(c: &mut Criterion) {
    let mut vec = black_box((0..32).step_by(4).cycle().take(1024).collect_vec());
    black_box(vec.sort_unstable());
    let vec = black_box(vec.iter().unique().cloned().collect_vec());

    c.bench_function("missing_integer_unique_sorted", move |b| {
        b.iter(|| {
            for elem in vec.iter().missing_integers_uqsort() {
                black_box(elem);
            }
        })
    });
}

fn missing_integer_uqsort_with_new(c: &mut Criterion) {
    let mut vec = black_box((0..32).step_by(4).cycle().take(1024).collect_vec());

    c.bench_function("missing_integer_with_new", move |b| {
        b.iter(|| {
            black_box(vec.sort_unstable());
            for elem in vec.iter().unique().missing_integers_uqsort() {
                black_box(elem);
            }
        })
    });
}

fn missing_integer(c: &mut Criterion) {
    let vec = black_box((0..32).step_by(4).cycle().take(1024).collect_vec());

    c.bench_function("missing_intgers", move |b| {
        b.iter(|| {
            for elem in vec.iter().missing_integers() {
                black_box(elem);
            }
        })
    });
}

criterion_group!(benches,  missing_integer, missing_integer_uqsort, missing_integer_uqsort_with_new);
criterion_main!(benches);
