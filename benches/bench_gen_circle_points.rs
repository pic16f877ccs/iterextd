use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iterextd::GenCirclePoints;

fn bench_create_circle_points_f32(c: &mut Criterion) {
    c.bench_function("Bench polar points f32", move |b| {
        b.iter(|| {
            for elem in black_box(GenCirclePoints::new(5.0_f32, 1034)) {
                black_box(elem);
            }
        })
    });
}

fn bench_create_circle_points_f64(c: &mut Criterion) {
    c.bench_function("Bench polar points f64", move |b| {
        b.iter(|| {
            for elem in black_box(GenCirclePoints::new(5.0_f64, 1034)) {
                black_box(elem);
            }
        })
    });
}

criterion_group!(benches, bench_create_circle_points_f32,  bench_create_circle_points_f64);
criterion_main!(benches);
