use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod color;

use crate::color::Rgb;
use crate::color::BLACK;

pub struct Params {
    pub width: usize,
    pub height: usize,
    pub xmax: f64,
    pub xmin: f64,
    pub ymax: f64,
    pub ymin: f64,
    pub max_iter: u32,
}

pub fn lsm(c: &[f64; 2], max_iter: u32) -> u32 {
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
pub unsafe fn lsm_v(cr: &[f64; 4], ci: &[f64; 4], max_iter: u32) -> [u32; 4] {
    let mut zr = [0.; 4];
    let mut zi = [0.; 4];
    let mut count = [0; 4];

    for _ in 0..max_iter {
        let mut mask = [0u32; 4];
        let mut sum = 0;

        for i in 0..4 {
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

pub fn render(buffer: &mut [u32], params: Params, palette: &Vec<Rgb>) {
    buffer
        .par_chunks_mut(params.width)
        .enumerate()
        .for_each(|(y, rows)| {
            #[cfg(target_feature = "avx2")]
            rows.chunks_mut(4).enumerate().for_each(|(x, v)| {
                let (cr, ci) = (
                    &[
                        ((x * 4) as f64 / params.width as f64)
                            * (params.xmax - params.xmin)
                            + params.xmin,
                        ((x * 4 + 1) as f64 / params.width as f64)
                            * (params.xmax - params.xmin)
                            + params.xmin,
                        ((x * 4 + 2) as f64 / params.width as f64)
                            * (params.xmax - params.xmin)
                            + params.xmin,
                        ((x * 4 + 3) as f64 / params.width as f64)
                            * (params.xmax - params.xmin)
                            + params.xmin,
                    ],
                    &[(y as f64 / params.height as f64)
                        * (params.ymax - params.ymin)
                        + params.ymin; 4],
                );
                let iterations = unsafe { lsm_v(cr, ci, params.max_iter) };
                iterations.iter().enumerate().for_each(|(i, x)| {
                    if *x == params.max_iter {
                        v[i] = BLACK.into();
                    } else {
                        v[i] = palette[*x as usize % palette.len()].into();
                    }
                })
            });

            #[cfg(not(target_feature = "avx2"))]
            rows.iter_mut().enumerate().for_each(|(x, pixel)| {
                let c = &[
                    (x as f64 / params.width as f64)
                        * (params.xmax - params.xmin)
                        + params.xmin,
                    (y as f64 / params.height as f64)
                        * (params.ymax - params.ymin)
                        + params.ymin,
                ];
                let iterations = lsm(c, params.max_iter);

                if iterations == params.max_iter {
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
