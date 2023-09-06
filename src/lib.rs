use rayon::prelude::*;
use std::mem::transmute;

const MAX_ITER: i32 = 1000;
const RADIUS_SQ: f64 = 4.0;
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

const XMIN: f64 = -2.5;
const XMAX: f64 = 1.0;
const YMIN: f64 = -1.0;
const YMAX: f64 = 1.0;

pub fn lsm(cr: f64, ci: f64) -> i32 {
    let mut zr = 0.0;
    let mut zi = 0.0;
    let mut zr2 = 0.0;
    let mut zi2 = 0.0;
    let mut iteration = 0;

    while (iteration < MAX_ITER) && (zr2 + zi2 < RADIUS_SQ) {
        zi = 2.0 * zr * zi + ci;
        zr = zr2 - zi2 + cr;
        zr2 = zr * zr;
        zi2 = zi * zi;
        iteration = iteration + 1;
    }
    iteration
}

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[target_feature(enable = "avx2")]
pub unsafe fn lsm_avx2(cr: __m256d, ci: __m256d) -> __m256d {
    let mut zr = _mm256_set1_pd(0.0);
    let mut zi = _mm256_set1_pd(0.0);

    let mut zr2 = _mm256_set1_pd(0.0);
    let mut zi2 = _mm256_set1_pd(0.0);

    let one = _mm256_set1_pd(1.0);
    let two = _mm256_set1_pd(2.0);
    let four = _mm256_set1_pd(4.0);

    let mut iterations = _mm256_set1_pd(0.0);

    for _ in 0..MAX_ITER {
        let mask = _mm256_cmp_pd::<_CMP_LT_OQ>(_mm256_add_pd(zr2, zi2), four);

        iterations = _mm256_add_pd(_mm256_and_pd(mask, one), iterations);

        if _mm256_movemask_pd(mask) == 0 {
            break;
        }

        zi = _mm256_add_pd(_mm256_mul_pd(two, _mm256_mul_pd(zr, zi)), ci);
        zr = _mm256_add_pd(_mm256_sub_pd(zr2, zi2), cr);

        zr2 = _mm256_mul_pd(zr, zr);
        zi2 = _mm256_mul_pd(zi, zi);
    }
    iterations
}

pub fn render_mandelbrot(palette: &Vec<(u8, u8, u8)>) -> Vec<u8> {
    let mut img_buffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 3];

    for y in 0..HEIGHT as u32 {
        for x in 0..WIDTH as u32 {
            let (cr, ci) = (
                (x as f64 / WIDTH as f64) * (XMAX - XMIN) + XMIN,
                (y as f64 / HEIGHT as f64) * (YMAX - YMIN) + YMIN,
            );
            let iterations = lsm(cr, ci);

            let pixel_r = (y as usize * WIDTH + x as usize) * 3;
            let pixel_g = (y as usize * WIDTH + x as usize) * 3 + 1;
            let pixel_b = (y as usize * WIDTH + x as usize) * 3 + 2;

            if iterations == MAX_ITER {
                img_buffer[pixel_r] = 0;
                img_buffer[pixel_g] = 0;
                img_buffer[pixel_b] = 0;
            } else {
                img_buffer[pixel_r] =
                    palette[iterations as usize % palette.len()].0;
                img_buffer[pixel_g] =
                    palette[iterations as usize % palette.len()].1;
                img_buffer[pixel_b] =
                    palette[iterations as usize % palette.len()].2;
            }
        }
    }
    img_buffer
}

pub fn render_parallel_mandelbrot(palette: &Vec<(u8, u8, u8)>) -> Vec<u8> {
    let mut img_buffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 3];

    img_buffer
        .par_chunks_exact_mut(WIDTH * 3)
        .enumerate()
        .for_each(|(y, rows)| {
            rows.chunks_exact_mut(12)
                .enumerate()
                .for_each(|(c, chunk)| {
                    let c = (c as f64) * 4.0;
                    let y = y as f64;

                    let cr = &mut [c, c + 1.0, c + 2.0, c + 3.0];
                    let ci = &mut [y; 4];

                    for (cr, ci) in cr.iter_mut().zip(ci.iter_mut()) {
                        *cr = (*cr / WIDTH as f64) * (XMAX - XMIN) + XMIN;
                        *ci = (*ci / HEIGHT as f64) * (YMAX - YMIN) + YMIN;
                    }

                    let iterations: [f64; 4] = unsafe {
                        transmute(lsm_avx2(transmute(*cr), transmute(*ci)))
                    };
                    chunk.chunks_exact_mut(3).enumerate().for_each(
                        |(t, triplet)| {
                            if iterations[t] == MAX_ITER as f64 {
                                triplet[0] = 0;
                                triplet[1] = 0;
                                triplet[2] = 0;
                            } else {
                                triplet[0] = palette
                                    [iterations[t] as usize % palette.len()]
                                .0;
                                triplet[1] = palette
                                    [iterations[t] as usize % palette.len()]
                                .1;
                                triplet[2] = palette
                                    [iterations[t] as usize % palette.len()]
                                .2;
                            }
                        },
                    )
                });
        });

    img_buffer
}

#[cfg(test)]
mod tests {}
