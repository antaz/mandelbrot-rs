use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod color;

use crate::color::Rgb;
use crate::color::BLACK;

const XMIN: f32 = -2.5;
const XMAX: f32 = 1.0;
const YMIN: f32 = -1.0;
const YMAX: f32 = 1.0;

pub fn lsm(c: &[f32; 2], max_iter: u32) -> u32 {
    let mut z = [0.; 2];
    let mut i = 0;

    while (i < max_iter) && (z[0] * z[0] + z[1] * z[1] < 4.) {
        (z[0], z[1]) =
            (z[0] * z[0] - z[1] * z[1] + c[0], 2. * z[0] * z[1] + c[1]);
        i += 1;
    }
    i
}

#[target_feature(enable = "avx2")]
pub unsafe fn lsm_v(cr: &[f32; 8], ci: &[f32; 8], max_iter: u32) -> [u32; 8] {
    let mut zr = [0.; 8];
    let mut zi = [0.; 8];
    let mut count = [0; 8];

    for _ in 0..max_iter {
        let mut mask = [0u32; 8];
        let mut sum = 0;

        for i in 0..8 {
            mask[i] = (zr[i] * zr[i] + zi[i] * zi[i] < 4.) as u32;
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

pub fn render(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    max_iter: u32,
    palette: &Vec<Rgb>,
) {
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
                let iterations = unsafe { lsm_v(cr, ci, max_iter) };
                iterations.iter().enumerate().for_each(|(i, x)| {
                    if *x == max_iter {
                        v[i] = BLACK.into();
                    } else {
                        v[i] = palette[*x as usize % palette.len()].into();
                    }
                })
            });

            #[cfg(not(target_feature = "avx2"))]
            rows.iter_mut().enumerate().for_each(|(x, pixel)| {
                let c = &[
                    (x as f32 / width as f32) * (XMAX - XMIN) + XMIN,
                    (y as f32 / height as f32) * (YMAX - YMIN) + YMIN,
                ];
                let iterations = lsm(c, max_iter);

                if iterations == max_iter {
                    *pixel = BLACK.into();
                } else {
                    *pixel =
                        palette[iterations as usize % palette.len()].into();
                }
            })
        });
}

/// Encode pixel buffer into PPM (Portable Pixel Map) format
/// Pixel buffer is in BGRA format (alpha is never used)
pub fn write_file(
    data: &[u32],
    path: &str,
    width: usize,
    height: usize,
) -> std::io::Result<()> {
    let path = Path::new(path);
    let mut file = File::create(&path)?;
    let header = format!("P6 {} {} 255 ", width, height);
    file.write(header.as_bytes())?;
    let buffer = data
        .iter()
        .map(|v| v.to_be_bytes()[1..4].iter().cloned().collect::<Vec<u8>>())
        .flatten()
        .collect::<Vec<u8>>();
    file.write(&buffer)?;
    Ok(())
}

#[cfg(test)]
mod tests {}
