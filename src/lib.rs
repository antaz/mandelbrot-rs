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
pub fn render_mandelbrot(
    palette: &Vec<(u8, u8, u8)>,
    width: usize,
    height: usize,
) -> Vec<u8> {
    let mut img_buffer: Vec<u8> = vec![0; width * height * 3];

    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let (cr, ci) = (
                (x as f64 / width as f64) * (XMAX - XMIN) + XMIN,
                (y as f64 / height as f64) * (YMAX - YMIN) + YMIN,
            );
            let iterations = lsm(cr, ci);

            let pixel_r = (y as usize * width + x as usize) * 3;
            let pixel_g = (y as usize * width + x as usize) * 3 + 1;
            let pixel_b = (y as usize * width + x as usize) * 3 + 2;

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

#[cfg(test)]
mod tests {}
