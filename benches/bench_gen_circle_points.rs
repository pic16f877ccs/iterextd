use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iterextd::GenCirclePoints;

fn bench_create_circle_points(c: &mut Criterion) {
    c.bench_function("Bench polar points", move |b| {
        b.iter(|| {
            for elem in black_box(GenCirclePoints::new(5.0, 1034)) {
                black_box(elem);
            }
        })
    });
}

criterion_group!(benches, bench_create_circle_points);
criterion_main!(benches);
