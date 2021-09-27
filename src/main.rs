use oxidfract::lsm;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// maximum iteration count
const MAX_ITER: i32 = 1000;
// width of the frame
const WIDTH: usize = 1280;
// height of the frame
const HEIGHT: usize = 720;

// mandelbrot range and domain
const XMIN: f64 = -2.5;
const XMAX: f64 = 1.0;
const YMIN: f64 = -1.0;
const YMAX: f64 = 1.0;

fn render_mandelbrot() -> Vec<u8> {
    let mut img_buffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 3];

    // main loop through all the pixels
    for y in 0..HEIGHT as u32 {
        for x in 0..WIDTH as u32 {
            // mapping the pixel coordinates to the Mandelbrot domain
            let (cr, ci) = (
                (x as f64 / WIDTH as f64) * (XMAX - XMIN) + XMIN,
                (y as f64 / HEIGHT as f64) * (YMAX - YMIN) + YMIN,
            );
            // calculate iterations
            let iterations = lsm(cr, ci);

            // set the pixels according to the iterations count
            let pixel_r = (y as usize * WIDTH + x as usize) * 3;
            let pixel_g = (y as usize * WIDTH + x as usize) * 3 + 1;
            let pixel_b = (y as usize * WIDTH + x as usize) * 3 + 2;

            if iterations == MAX_ITER {
                img_buffer[pixel_r] = 0;
                img_buffer[pixel_g] = 0;
                img_buffer[pixel_b] = 0;
            } else {
                img_buffer[pixel_r] = 255;
                img_buffer[pixel_g] = 255;
                img_buffer[pixel_b] = 255;
            }
        }
    }
    img_buffer
}

fn write_file(data: Vec<u8>, filename: &str) -> std::io::Result<()> {
    let path = Path::new(filename);
    let mut file = File::create(&path)?;
    let header = format!("P6 {} {} 255\n", WIDTH, HEIGHT);
    file.write(header.as_bytes())?;
    file.write(&data)?;
    Ok(())
}
fn main() -> Result<(), std::io::Error> {
    let fractal = render_mandelbrot();
    write_file(fractal, "out/fractal.ppm")
}
