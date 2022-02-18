use oxidfract::{lsm, lsm_avx2};
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::mem::transmute;
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

fn render_mandelbrot(palette: Vec<(u8, u8, u8)>) -> Vec<u8> {
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

fn render_parallel_mandelbrot(palette: Vec<(u8, u8, u8)>) -> Vec<u8> {
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

fn write_file(data: Vec<u8>, filename: &str) -> std::io::Result<()> {
    let path = Path::new(filename);
    let mut file = File::create(&path)?;
    let header = format!("P6 {} {} 255\n", WIDTH, HEIGHT);
    file.write(header.as_bytes())?;
    file.write(&data)?;
    Ok(())
}
fn main() -> Result<(), std::io::Error> {
    let palette = vec![
        (38, 70, 83),
        (42, 157, 143),
        (233, 196, 106),
        (244, 162, 97),
        (231, 111, 81),
    ];
    let fractal = render_parallel_mandelbrot(palette);
    write_file(fractal, "out/fractal.ppm")
}
