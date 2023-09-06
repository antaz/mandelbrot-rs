use mandelbrot_rs::render_mandelbrot;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn write_file(
    data: Vec<u8>,
    path: &str,
    width: usize,
    height: usize,
) -> std::io::Result<()> {
    let path = Path::new(path);
    let mut file = File::create(&path)?;
    let header = format!("P6 {} {} 255\n", width, height);
    file.write(header.as_bytes())?;
    file.write(&data)?;
    Ok(())
}
fn main() -> () {
    let dim = std::env::args().nth(1).expect("Expected dimensions");
    let mut dim = dim.split("x");

    let width: usize = dim.next().unwrap().parse().unwrap();
    let height: usize = dim.next().unwrap().parse().unwrap();
    let path = std::env::args().nth(2).expect("Expected path");

    let palette = vec![
        (38, 70, 83),
        (42, 157, 143),
        (233, 196, 106),
        (244, 162, 97),
        (231, 111, 81),
    ];

    let data = render_mandelbrot(&palette, width, height);
    write_file(data, &path, width, height).unwrap();
}
