use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iterextd::Scaler;

fn bench_scaling_value_types_u32_u32_u32(c: &mut Criterion) {
    c.bench_function("Bench scaling value u32, u32, u32", move |b| {
        b.iter(|| {
            for elem in black_box((u32::MIN..=1024u32).scaling::<u32>(u32::MIN..=1024u32)) {
                black_box(elem);
            }
        })
    });
}

fn bench_scaling_value_types_u32_u32_i32(c: &mut Criterion) {
    c.bench_function("Bench scaling value u32, u32, i32", move |b| {
        b.iter(|| {
            for elem in
                black_box((u32::MIN..=1024u32).scaling::<u32>(i32::MIN..=i32::MIN + 1024i32))
            {
                black_box(elem);
            }
        })
    });
}

fn bench_scaling_value_types_i32_u32_i32(c: &mut Criterion) {
    c.bench_function("Bench scaling value i32, u32, i32", move |b| {
        b.iter(|| {
            for elem in
                black_box((i32::MIN..=i32::MIN + 1024).scaling::<u32>(i32::MIN..=i32::MIN + 1024))
            {
                black_box(elem);
            }
        })
    });
}

fn bench_scaling_value_types_u32_u32_u32_new(c: &mut Criterion) {
    c.bench_function("Bench scaling value u32, u32, u32 new", move |b| {
        let iter = (u32::MIN..=1024).scaling::<u32>(u32::MIN..=1024);
        b.iter(|| {
            for elem in black_box(iter.clone()) {
                black_box(elem);
            }
        })
    });
}

fn bench_scaling_value_types_u32_u32_i32_new(c: &mut Criterion) {
    c.bench_function("Bench scaling value u32, u32, i32 new", move |b| {
        let iter = (u32::MIN..=1024).scaling::<u32>(i32::MIN..=i32::MIN + 1024);
        b.iter(|| {
            for elem in black_box(iter.clone()) {
                black_box(elem);
            }
        })
    });
}

fn bench_scaling_value_types_i32_u32_i32_new(c: &mut Criterion) {
    c.bench_function("Bench scaling value i32, u32, i32 new", move |b| {
        let iter = (i32::MIN..=i32::MIN + 1024).scaling::<u32>(i32::MIN..=i32::MIN + 1024);
        b.iter(|| {
            for elem in black_box(iter.clone()) {
                black_box(elem);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_scaling_value_types_u32_u32_u32,
    bench_scaling_value_types_u32_u32_i32,
    bench_scaling_value_types_i32_u32_i32,
    bench_scaling_value_types_u32_u32_u32_new,
    bench_scaling_value_types_u32_u32_i32_new,
    bench_scaling_value_types_i32_u32_i32_new,
);
criterion_main!(benches);
