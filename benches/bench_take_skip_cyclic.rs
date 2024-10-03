use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iterextd::IterExtd;

fn bench_take_skip_cyclic(c: &mut Criterion) {
    c.bench_function("take_skip_cyclic", move |b| {
        b.iter(|| {
            for elem in black_box(0..1024).take_skip_cyclic(1, 1) {
                black_box(elem);
            }
        })
    });
}

fn bench_std_step(c: &mut Criterion) {
    c.bench_function("std_adapter_step", move |b| {
        b.iter(|| {
            for elem in black_box(0..1024).step_by(2) {
                black_box(elem);
            }
        })
    });
}

criterion_group!(benches, bench_take_skip_cyclic, bench_std_step);
criterion_main!(benches);
