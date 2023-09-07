use rayon::prelude::*;

const MAX_ITER: i32 = 1000;
const RADIUS_SQ: f64 = 4.0;

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
pub fn render_mandelbrot(buffer: &mut [u32], width: usize, height: usize) {
    buffer
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, rows)| {
            rows.iter_mut().enumerate().for_each(|(x, pixel)| {
                let (cr, ci) = (
                    (x as f64 / width as f64) * (XMAX - XMIN) + XMIN,
                    (y as f64 / height as f64) * (YMAX - YMIN) + YMIN,
                );
                let iterations = lsm(cr, ci);

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
