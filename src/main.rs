use mandelbrot_rs::render_mandelbrot;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn write_file(
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
        .map(|v| v.to_le_bytes()[0..3].iter().cloned().collect::<Vec<u8>>())
        .flatten()
        .collect::<Vec<u8>>();
    file.write(&buffer)?;
    Ok(())
}
fn main() -> () {
    let dim = std::env::args().nth(1).expect("Expected dimensions");
    let mut dim = dim.split("x");

    let width: usize = dim.next().unwrap().parse().unwrap();
    let height: usize = dim.next().unwrap().parse().unwrap();
    let path = std::env::args().nth(2).expect("Expected path");

    let mut buffer = vec![0u32; width * height];
    render_mandelbrot(&mut buffer, width, height);
    write_file(&buffer, &path, width, height).unwrap();
}
