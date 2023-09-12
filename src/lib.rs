use rayon::prelude::*;

const MAX_ITER: i32 = 1000;
const RADIUS_SQ: f32 = 4.0;

const XMIN: f32 = -2.5;
const XMAX: f32 = 1.0;
const YMIN: f32 = -1.0;
const YMAX: f32 = 1.0;

pub fn lsm(c: &[f32; 2]) -> i32 {
    let mut z = [0.; 2];
    let mut i = 0;

    while (i < MAX_ITER) && (z[0] + z[1] < RADIUS_SQ) {
        (z[0], z[1]) =
            (z[0] * z[0] - z[1] * z[1] + c[0], 2. * z[0] * z[1] + c[1]);
        i += 1;
    }
    i
}
pub fn render_mandelbrot(buffer: &mut [u32], width: usize, height: usize) {
    buffer
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, rows)| {
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
