#[allow(dead_code)]

// maximum iteration count
pub const MAX_ITER: u32 = 1000;
// escaoe radius squared
const RADIUS_SQ: f64 = 4.0;

// scalar implementation of Level Set Method
pub fn lsm(cr: f64, ci: f64) -> u32 {
    // z (real and imaginary parts init)
    let mut zr = 0.0;
    let mut zi = 0.0;
    // z^2 (real and imaginary parts init)
    let mut zr2 = 0.0;
    let mut zi2 = 0.0;
    // iteration count
    let mut iteration = 0;

    while (iteration < MAX_ITER) && (zr2 + zi2 < RADIUS_SQ) {
        // update z
        zi = 2.0 * zr * zi + ci;
        zr = zr2 - zi2 + cr;
        // update z^2
        zr2 = zr * zr;
        zi2 = zi * zi;
        // and update the iteration count
        iteration = iteration + 1;
    }
    iteration
}

// Auto vectorization x4 implementation of Level Set Method
pub fn lsm4(cr: [f64; 4], ci: [f64; 4]) -> [u32; 4] {
    // z (real and imaginary parts init)
    let mut zr = [0.0; 4];
    let mut zi = [0.0; 4];
    // z^2 (real and imaginary parts init)
    let mut zr2 = [0.0; 4];
    let mut zi2 = [0.0; 4];
    // iteration count
    let mut iteration = [0.0; 4];

    for _ in 0..MAX_ITER {
        let mask = [
            if zr2[0] + zi2[0] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[1] + zi2[1] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[2] + zi2[2] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[3] + zi2[3] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
        ];
        if mask == [0.0; 4] {
            break;
        }
        // and update the iteration count
        iteration[0] = iteration[0] + mask[0];
        iteration[1] = iteration[1] + mask[1];
        iteration[2] = iteration[2] + mask[2];
        iteration[3] = iteration[3] + mask[3];
        // update z
        zi[0] = 2.0 * zr[0] * zi[0] + ci[0];
        zi[1] = 2.0 * zr[1] * zi[1] + ci[1];
        zi[2] = 2.0 * zr[2] * zi[2] + ci[2];
        zi[3] = 2.0 * zr[3] * zi[3] + ci[3];
        zr[0] = zr2[0] - zi2[0] + cr[0];
        zr[1] = zr2[1] - zi2[1] + cr[1];
        zr[2] = zr2[2] - zi2[2] + cr[2];
        zr[3] = zr2[3] - zi2[3] + cr[3];
        // update z^2
        zr2[0] = zr[0] * zr[0];
        zr2[1] = zr[1] * zr[1];
        zr2[2] = zr[2] * zr[2];
        zr2[3] = zr[3] * zr[3];
        zi2[0] = zi[0] * zi[0];
        zi2[1] = zi[1] * zi[1];
        zi2[2] = zi[2] * zi[2];
        zi2[3] = zi[3] * zi[3];
    }
    [
        iteration[0] as u32,
        iteration[1] as u32,
        iteration[2] as u32,
        iteration[3] as u32,
    ]
}

// Auto vectorization x8 implementation of Level Set Method
pub fn lsm8(cr: [f64; 8], ci: [f64; 8]) -> [u32; 8] {
    // z (real and imaginary parts init)
    let mut zr = [0.0; 8];
    let mut zi = [0.0; 8];
    // z^2 (real and imaginary parts init)
    let mut zr2 = [0.0; 8];
    let mut zi2 = [0.0; 8];
    // iteration count
    let mut iteration = [0.0; 8];

    for _ in 0..MAX_ITER {
        let mask = [
            if zr2[0] + zi2[0] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[1] + zi2[1] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[2] + zi2[2] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[3] + zi2[3] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[4] + zi2[4] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[5] + zi2[5] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[6] + zi2[6] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
            if zr2[7] + zi2[7] < RADIUS_SQ {
                1.0
            } else {
                0.0
            },
        ];
        if mask == [0.0; 8] {
            break;
        }
        // and update the iteration count
        iteration[0] = iteration[0] + mask[0];
        iteration[1] = iteration[1] + mask[1];
        iteration[2] = iteration[2] + mask[2];
        iteration[3] = iteration[3] + mask[3];
        iteration[4] = iteration[4] + mask[4];
        iteration[5] = iteration[5] + mask[5];
        iteration[6] = iteration[6] + mask[6];
        iteration[7] = iteration[7] + mask[7];
        // update z
        zi[0] = 2.0 * zr[0] * zi[0] + ci[0];
        zi[1] = 2.0 * zr[1] * zi[1] + ci[1];
        zi[2] = 2.0 * zr[2] * zi[2] + ci[2];
        zi[3] = 2.0 * zr[3] * zi[3] + ci[3];
        zi[4] = 2.0 * zr[4] * zi[4] + ci[4];
        zi[5] = 2.0 * zr[5] * zi[5] + ci[5];
        zi[6] = 2.0 * zr[6] * zi[6] + ci[6];
        zi[7] = 2.0 * zr[7] * zi[7] + ci[7];
        zr[0] = zr2[0] - zi2[0] + cr[0];
        zr[1] = zr2[1] - zi2[1] + cr[1];
        zr[2] = zr2[2] - zi2[2] + cr[2];
        zr[3] = zr2[3] - zi2[3] + cr[3];
        zr[4] = zr2[4] - zi2[4] + cr[4];
        zr[5] = zr2[5] - zi2[5] + cr[5];
        zr[6] = zr2[6] - zi2[6] + cr[6];
        zr[7] = zr2[7] - zi2[7] + cr[7];
        // update z^2
        zr2[0] = zr[0] * zr[0];
        zr2[1] = zr[1] * zr[1];
        zr2[2] = zr[2] * zr[2];
        zr2[3] = zr[3] * zr[3];
        zr2[4] = zr[4] * zr[4];
        zr2[5] = zr[5] * zr[5];
        zr2[6] = zr[6] * zr[6];
        zr2[7] = zr[7] * zr[7];
        zi2[0] = zi[0] * zi[0];
        zi2[1] = zi[1] * zi[1];
        zi2[2] = zi[2] * zi[2];
        zi2[3] = zi[3] * zi[3];
        zi2[4] = zi[4] * zi[4];
        zi2[5] = zi[5] * zi[5];
        zi2[6] = zi[6] * zi[6];
        zi2[7] = zi[7] * zi[7];
    }
    [
        iteration[0] as u32,
        iteration[1] as u32,
        iteration[2] as u32,
        iteration[3] as u32,
        iteration[4] as u32,
        iteration[5] as u32,
        iteration[6] as u32,
        iteration[7] as u32,
    ]
}

#[cfg(feature = "lsm_avx2")]
mod avx2 {
    use super::MAX_ITER;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;
    use std::mem::transmute;

    #[target_feature(enable = "avx2")]
    unsafe fn lsm(cr: __m256d, ci: __m256d) -> __m256d {
        // z (real and imaginary parts init)
        let mut zr = _mm256_set1_pd(0.0);
        let mut zi = _mm256_set1_pd(0.0);

        // z^2 (real and imaginary parts init)
        let mut zr2 = _mm256_set1_pd(0.0);
        let mut zi2 = _mm256_set1_pd(0.0);

        // useful constants
        let one = _mm256_set1_pd(1.0);
        let two = _mm256_set1_pd(2.0);
        let four = _mm256_set1_pd(4.0);

        // iteration counts
        let mut iterations = _mm256_set1_pd(0.0);

        for _ in 0..MAX_ITER {
            // comparison mask of the magnitudes with the escape radius
            let mask = _mm256_cmp_pd::<_CMP_LT_OQ>(_mm256_add_pd(zr2, zi2), four);

            // break if all values exceeded the threshold
            if _mm256_movemask_pd(mask) == 0 {
                break;
            }

            // update the iteration counts
            iterations = _mm256_add_pd(_mm256_and_pd(mask, one), iterations);

            // update z
            zi = _mm256_add_pd(_mm256_mul_pd(two, _mm256_mul_pd(zr, zi)), ci);
            zr = _mm256_add_pd(_mm256_sub_pd(zr2, zi2), cr);

            // update z^2
            zr2 = _mm256_mul_pd(zr, zr);
            zi2 = _mm256_mul_pd(zi, zi);
        }
        iterations
    }

    pub fn lsm_avx2(cr: [f64; 4], ci: [f64; 4]) -> [u32; 4] {
        let f: [f64; 4] = unsafe { transmute(lsm(transmute(cr), transmute(ci))) };
        [f[0] as u32, f[1] as u32, f[2] as u32, f[3] as u32]
    }
}
#[cfg(feature = "lsm_avx2")]
pub use avx2::lsm_avx2;

#[cfg(test)]
mod tests {}
