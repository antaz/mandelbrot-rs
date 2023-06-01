#[cfg(feature = "lsm")]
use oxidfract::lsm;
#[cfg(feature = "lsm4")]
use oxidfract::lsm4;
#[cfg(feature = "lsm8")]
use oxidfract::lsm8;
#[cfg(feature = "lsm_avx2")]
use oxidfract::lsm_avx2;
use oxidfract::MAX_ITER;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

// width of the frame
const WIDTH: usize = 25600;
// height of the frame
const HEIGHT: usize = 25600;

// mandelbrot range and domain
const XMIN: f64 = -2.5;
const XMAX: f64 = 1.0;
const YMIN: f64 = -1.0;
const YMAX: f64 = 1.0;

#[cfg(not(any(
    feature = "lsm_avx2",
    feature = "lsm",
    feature = "lsm4",
    feature = "lsm8"
)))]
compile_error!(
    "at least one feature \"lsm_avx2\", \"lsm\" , \"lsm4\" or \"lsm8\" needs to be enabled"
);

#[cfg(feature = "lsm")]
fn render_mandelbrot(palette: Vec<(u8, u8, u8)>) -> Vec<u8> {
    let mut img_buffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 3];

    #[cfg(feature = "rayon")]
    let buf_iter = img_buffer.par_chunks_exact_mut(WIDTH * 3);
    #[cfg(not(feature = "rayon"))]
    let buf_iter = img_buffer.chunks_exact_mut(WIDTH * 3);

    buf_iter.enumerate().for_each(|(y, rows)| {
        let ci = (y as f64 / HEIGHT as f64) * (YMAX - YMIN) + YMIN;

        rows.chunks_exact_mut(3).enumerate().for_each(|(x, pixel)| {
            let cr = (x as f64 / WIDTH as f64) * (XMAX - XMIN) + XMIN;
            // calculate iterations
            let iterations = lsm(cr, ci);
            if iterations == MAX_ITER {
                pixel[0] = 0;
                pixel[1] = 0;
                pixel[2] = 0;
            } else {
                pixel[0] = palette[iterations as usize % palette.len()].0;
                pixel[1] = palette[iterations as usize % palette.len()].1;
                pixel[2] = palette[iterations as usize % palette.len()].2;
            }
        });
    });

    img_buffer
}

#[cfg(feature = "lsm8")]
fn render_mandelbrot(palette: Vec<(u8, u8, u8)>) -> Vec<u8> {
    let mut img_buffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 3];

    #[cfg(feature = "rayon")]
    let buf_iter = img_buffer.par_chunks_exact_mut(WIDTH * 3);
    #[cfg(not(feature = "rayon"))]
    let buf_iter = img_buffer.chunks_exact_mut(WIDTH * 3);

    buf_iter.enumerate().for_each(|(y, rows)| {
        let ci = (y as f64 / HEIGHT as f64) * (YMAX - YMIN) + YMIN;
        let ci = &mut [ci; 8];

        rows.chunks_exact_mut(8 * 3)
            .enumerate()
            .for_each(|(c, chunk)| {
                let c = (c as f64) * 8.0;

                let cr = &mut [
                    c,
                    c + 1.0,
                    c + 2.0,
                    c + 3.0,
                    c + 4.0,
                    c + 5.0,
                    c + 6.0,
                    c + 7.0,
                ];

                for cr in cr.iter_mut() {
                    *cr = (*cr / WIDTH as f64) * (XMAX - XMIN) + XMIN;
                }

                let iterations: [u32; 8] = lsm8(*cr, *ci);

                chunk
                    .chunks_exact_mut(3)
                    .enumerate()
                    .for_each(|(t, triplet)| {
                        if iterations[t] == MAX_ITER {
                            triplet[0] = 0;
                            triplet[1] = 0;
                            triplet[2] = 0;
                        } else {
                            triplet[0] = palette[iterations[t] as usize % palette.len()].0;
                            triplet[1] = palette[iterations[t] as usize % palette.len()].1;
                            triplet[2] = palette[iterations[t] as usize % palette.len()].2;
                        }
                    })
            });
    });

    img_buffer
}

#[cfg(any(feature = "lsm_avx2", feature = "lsm4"))]
fn render_mandelbrot(palette: Vec<(u8, u8, u8)>) -> Vec<u8> {
    let mut img_buffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 3];

    #[cfg(feature = "rayon")]
    let buf_iter = img_buffer.par_chunks_exact_mut(WIDTH * 3);
    #[cfg(not(feature = "rayon"))]
    let buf_iter = img_buffer.chunks_exact_mut(WIDTH * 3);

    buf_iter.enumerate().for_each(|(y, rows)| {
        let ci = (y as f64 / HEIGHT as f64) * (YMAX - YMIN) + YMIN;
        let ci = &mut [ci; 4];

        rows.chunks_exact_mut(12)
            .enumerate()
            .for_each(|(c, chunk)| {
                let c = (c as f64) * 4.0;

                let cr = &mut [c, c + 1.0, c + 2.0, c + 3.0];

                for cr in cr.iter_mut() {
                    *cr = (*cr / WIDTH as f64) * (XMAX - XMIN) + XMIN;
                }

                #[cfg(feature = "lsm4")]
                let iterations: [u32; 4] = lsm4(*cr, *ci);
                #[cfg(feature = "lsm_avx2")]
                let iterations: [u32; 4] = lsm_avx2(*cr, *ci);

                chunk
                    .chunks_exact_mut(3)
                    .enumerate()
                    .for_each(|(t, triplet)| {
                        if iterations[t] == MAX_ITER {
                            triplet[0] = 0;
                            triplet[1] = 0;
                            triplet[2] = 0;
                        } else {
                            triplet[0] = palette[iterations[t] as usize % palette.len()].0;
                            triplet[1] = palette[iterations[t] as usize % palette.len()].1;
                            triplet[2] = palette[iterations[t] as usize % palette.len()].2;
                        }
                    })
            });
    });

    img_buffer
}

#[cfg(feature = "write_file")]
fn write_file(data: Vec<u8>, filename: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

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
    let fractal = render_mandelbrot(palette);

    #[cfg(feature = "blake3")]
    {
        let hash = blake3::hash(&fractal);
        println!("hash = {hash:?}");
    }

    #[cfg(feature = "write_file")]
    write_file(fractal, "out/fractal.ppm")?;

    Ok(())
}
