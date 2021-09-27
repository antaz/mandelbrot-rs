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

#[cfg(test)]
mod tests {}
