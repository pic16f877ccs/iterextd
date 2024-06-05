use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use iterextd::IterExtd;

fn extrapolate_half_step(c: &mut Criterion) {

    c.bench_function("Extrapolate float half step", |b| {
        b.iter(|| {
            for elem in black_box([1.0, 1.5]).into_iter().extrapolate().take(black_box(1024)) {
                black_box(elem);
            }
        })
    });
}

fn extrapolate(c: &mut Criterion) {

    c.bench_function("Extrapolate array with two elements", |b| {
        b.iter(|| {
            for elem in black_box([1, 2]).into_iter().extrapolate().take(black_box(1024)) {
                black_box(elem);
            }
        })
    });
}

fn std_range(c: &mut Criterion) {

    c.bench_function("Std range", |b| {
        b.iter(|| {
            for elem in black_box(1..1024) {
                black_box(elem);
            }
        })
    });
}

fn extrapolate_with_step_two(c: &mut Criterion) {

    c.bench_function("Extrapolate array with step two", |b| {
        b.iter(|| {
            for elem in black_box([1, 3]).into_iter().extrapolate().take(black_box(1024)) {
                black_box(elem);
            }
        })
    });
}

fn std_range_with_step_two(c: &mut Criterion) {

    c.bench_function("Std range step two", |b| {
        b.iter(|| {
            for elem in black_box(1..2049).step_by(black_box(2)) {
                black_box(elem);
            }
        })
    });
}

fn extrapolate_with_chain(c: &mut Criterion) {

    c.bench_function("Extrapolate array chain", |b| {
        b.iter(|| {
            for elem in black_box([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).into_iter().extrapolate().take(black_box(1024)) {
                black_box(elem);
            }
        })
    });
}

fn std_range_with_chain(c: &mut Criterion) {

    c.bench_function("Std range chain", |b| {
        b.iter(|| {
            for elem in black_box([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).into_iter().chain(black_box(11..1025)) {
                black_box(elem);
            }
        })
    });
}

fn extrapolate_with_chain_step_two(c: &mut Criterion) {

    c.bench_function("Extrapolate array chain with step two", |b| {
        b.iter(|| {
            for elem in black_box([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12]).into_iter().extrapolate().take(black_box(1024)) {
                black_box(elem);
            }
        })
    });
}

fn std_range_with_chain_step_two(c: &mut Criterion) {

    c.bench_function("Std range chain with step two", |b| {
        b.iter(|| {
            for elem in black_box([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12]).into_iter().chain(black_box(14..2039).step_by(black_box(2))) {
                black_box(elem);
            }
        })
    });
}

criterion_group!(benches, extrapolate_half_step, extrapolate, std_range, extrapolate_with_step_two, std_range_with_step_two, extrapolate_with_chain, std_range_with_chain, extrapolate_with_chain_step_two, std_range_with_chain_step_two);
criterion_main!(benches);
