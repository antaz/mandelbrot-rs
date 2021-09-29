#[allow(dead_code)]

// maximum iteration count
const MAX_ITER: i32 = 1000;
// escaoe radius squared
const RADIUS_SQ: f64 = 4.0;

// scalar implementation of Level Set Method
pub fn lsm(cr: f64, ci: f64) -> i32 {
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

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[target_feature(enable = "avx2")]
pub unsafe fn lsm_avx2(cr: __m256d, ci: __m256d) -> __m256d {
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

        // update the iteration counts
        iterations = _mm256_add_pd(_mm256_and_pd(mask, one), iterations);

        // break if all values exceeded the threshold
        if _mm256_movemask_pd(mask) == 0 {
            break;
        }

        // update z
        zi = _mm256_add_pd(_mm256_mul_pd(two, _mm256_mul_pd(zr, zi)), ci);
        zr = _mm256_add_pd(_mm256_sub_pd(zr2, zi2), cr);

        // update z^2
        zr2 = _mm256_mul_pd(zr, zr);
        zi2 = _mm256_mul_pd(zi, zi);
    }
    iterations
}

#[cfg(test)]
mod tests {}
