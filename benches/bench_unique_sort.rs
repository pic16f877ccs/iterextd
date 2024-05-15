use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use iterextd::IterExtd;

fn unique_sorted(c: &mut Criterion) {
    let vec = black_box((0..32).cycle().take(1024).collect_vec());

    c.bench_function("unique_sorted", move |b| {
        b.iter(|| {
            for elem in vec.iter().unique_sorted() {
                black_box(elem);
            }
        })
    });
}

fn unique_sorted_itertool(c: &mut Criterion) {
    let vec = black_box((0..32).cycle().take(1024).collect_vec());

    c.bench_function("unique_sorted_itertool", move |b| {
        b.iter(|| {
            for elem in vec.iter().unique().sorted() {
                black_box(elem);
            }
        })
    });
}

criterion_group!(benches,   unique_sorted, unique_sorted_itertool);
criterion_main!(benches);

