#![feature(test)]

extern crate test;

use mandelbrot_rs::color::stretch;
use mandelbrot_rs::color::XAOS;
use mandelbrot_rs::{render, Params};
use test::Bencher;

#[bench]
fn render_bench(b: &mut Bencher) {
    let mut buffer = vec![0u32; 1280 * 720];
    let palette = stretch(8, &XAOS);
    let params = Params {
        width: 1280,
        height: 720,
        xmax: 1.0,
        xmin: -2.5,
        ymax: 1.0,
        ymin: -1.0,
        max_iter: 512,
    };

    b.iter(|| {
        render(
            test::black_box(&mut buffer),
            test::black_box(params),
            test::black_box(&palette),
        );
    });
}
