# Mandelbrot Fractal Renderer

This is a [Mandelbrot](https://en.wikipedia.org/wiki/Mandelbrot_set) fractal renderer written in Rust.

![]()

## Features
- Multithreading with [Rayon](https://docs.rs/rayon/latest/rayon/)
- SIMD with both AVX2

## Benchmarks
benchmark results on an i3-5005U CPU @ 2.00GHz

| Version         | CPU utilized | Instructions (B) | Time elapsed (s) | 
| --------------- | ------------ | ---------------- | ---------------- |
| non-parallel    | 0.999        | 3.505            | 1.204244         |
| parallel        | 3.839        | 3.485            | 0.36724          |
| parallel-avx2   | 3.792        | 0.864            | 0.121628         | 
