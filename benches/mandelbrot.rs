use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mandelbrot_rs::render_mandelbrot;

pub fn render_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench");
    group.sample_size(10);
    let mut buffer = vec![1280 * 720];

    group.bench_function("render_mandelbrot 1280 720", |b| {
        b.iter(|| {
            render_mandelbrot(
                black_box(&mut buffer),
                black_box(1280),
                black_box(720),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, render_bench);
criterion_main!(benches);
