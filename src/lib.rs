use rayon::prelude::*;

const MAX_ITER: u32 = 1000;
const RADIUS_SQ: f32 = 4.0;

const XMIN: f32 = -2.5;
const XMAX: f32 = 1.0;
const YMIN: f32 = -1.0;
const YMAX: f32 = 1.0;

pub fn lsm(c: &[f32; 2]) -> u32 {
    let mut z = [0.; 2];
    let mut i = 0;

    while (i < MAX_ITER) && (z[0] * z[0] + z[1] * z[1] < RADIUS_SQ) {
        (z[0], z[1]) =
            (z[0] * z[0] - z[1] * z[1] + c[0], 2. * z[0] * z[1] + c[1]);
        i += 1;
    }
    i
}

#[target_feature(enable = "avx2")]
pub unsafe fn lsm_v(cr: &[f32; 8], ci: &[f32; 8]) -> [u32; 8] {
    let mut zr = [0.; 8];
    let mut zi = [0.; 8];
    let mut count = [0; 8];

    for _ in 0..MAX_ITER {
        let mut mask = [0u32; 8];
        let mut sum = 0;

        for i in 0..8 {
            mask[i] = (zr[i] * zr[i] + zi[i] * zi[i] < RADIUS_SQ) as u32;
            count[i] += mask[i];

            (zr[i], zi[i]) = (
                zr[i] * zr[i] - zi[i] * zi[i] + cr[i],
                2. * zr[i] * zi[i] + ci[i],
            );

            sum += mask[i];
        }

        if sum == 0 {
            break;
        }
    }

    count
}

pub fn render_mandelbrot(buffer: &mut [u32], width: usize, height: usize) {
    buffer
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, rows)| {
            #[cfg(target_feature = "avx2")]
            rows.chunks_mut(8).enumerate().for_each(|(x, v)| {
                let (cr, ci) = (
                    &[
                        ((x * 8) as f32 / width as f32) * (XMAX - XMIN) + XMIN,
                        ((x * 8 + 1) as f32 / width as f32) * (XMAX - XMIN)
                            + XMIN,
                        ((x * 8 + 2) as f32 / width as f32) * (XMAX - XMIN)
                            + XMIN,
                        ((x * 8 + 3) as f32 / width as f32) * (XMAX - XMIN)
                            + XMIN,
                        ((x * 8 + 4) as f32 / width as f32) * (XMAX - XMIN)
                            + XMIN,
                        ((x * 8 + 5) as f32 / width as f32) * (XMAX - XMIN)
                            + XMIN,
                        ((x * 8 + 6) as f32 / width as f32) * (XMAX - XMIN)
                            + XMIN,
                        ((x * 8 + 7) as f32 / width as f32) * (XMAX - XMIN)
                            + XMIN,
                    ],
                    &[(y as f32 / height as f32) * (YMAX - YMIN) + YMIN; 8],
                );
                let iterations = unsafe { lsm_v(cr, ci) };
                iterations.iter().enumerate().for_each(|(i, x)| {
                    if *x == MAX_ITER {
                        v[i] = 0 | (0 << 8) | (0 << 16);
                    } else {
                        v[i] = 255 | (255 << 8) | (255 << 16);
                    }
                })
            });

            #[cfg(not(target_feature = "avx2"))]
            rows.iter_mut().enumerate().for_each(|(x, pixel)| {
                let c = &[
                    (x as f32 / width as f32) * (XMAX - XMIN) + XMIN,
                    (y as f32 / height as f32) * (YMAX - YMIN) + YMIN,
                ];
                let iterations = lsm(c);

                if iterations == MAX_ITER {
                    *pixel = 0 | (0 << 8) | (0 << 16);
                } else {
                    *pixel = 255 | (255 << 8) | (255 << 16);
                }
            })
        });
}
#[cfg(test)]
mod tests {}
