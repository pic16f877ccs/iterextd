use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iterextd::{CircleBresenhamSeq, IterExtd};

fn bench_create_bresenham_circle(c: &mut Criterion) {
    c.bench_function("Bench_bresenham_circle", move |b| {
        b.iter(|| {
            for elem in black_box(CircleBresenhamSeq::new(2500_u16).offset(2500_i32, 2500_i32)) {
                black_box(elem);
            }
        })
    });
}

criterion_group!(benches, bench_create_bresenham_circle);
criterion_main!(benches);
