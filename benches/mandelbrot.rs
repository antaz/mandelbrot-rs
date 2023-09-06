use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mandelbrot_rs::lsm;
use mandelbrot_rs::render_mandelbrot;

pub fn lsm_bench(c: &mut Criterion) {
    c.bench_function("lsm 2.3 -4.5", |b| {
        b.iter(|| lsm(black_box(2.3), black_box(-4.5)))
    });
}

pub fn render_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench");
    group.sample_size(10);
    let palette = vec![
        (38, 70, 83),
        (42, 157, 143),
        (233, 196, 106),
        (244, 162, 97),
        (231, 111, 81),
    ];

    group.bench_function("render_mandelbrot 1280 720", |b| {
        b.iter(|| {
            render_mandelbrot(
                black_box(&palette),
                black_box(1280),
                black_box(720),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, lsm_bench, render_bench);
criterion_main!(benches);
