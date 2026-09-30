use clap::Parser;
use mandelbrot_rs::cli::Args;
use mandelbrot_rs::color::{stretch, FIRE, XAOS};
use mandelbrot_rs::video::render_video;
use mandelbrot_rs::{render, write_file, Params};

fn main() {
    let a = Args::parse();
    let frames = if a.frames == 0 && a.output.ends_with(".mp4") { 150 } else { a.frames };

    let base_palette = match a.palette.to_lowercase().as_str() {
        "fire" => &FIRE,
        _ => &XAOS,
    };

    if frames > 1 {
        render_video(&a.output, a.width, a.height, a.iterations, a.zoom, a.cx, a.cy, frames, a.zoom_speed, &a.palette)
            .unwrap_or_else(|e| { eprintln!("Error: Failed to generate video ({})", e); std::process::exit(1); });
    } else {
        let mut buffer = vec![0u32; a.width * a.height];
        let xr = 3.5 / a.zoom;
        let yr = xr * (a.height as f64 / a.width as f64);
        let params = Params {
            width: a.width, height: a.height,
            xmax: a.cx + xr / 2.0, xmin: a.cx - xr / 2.0,
            ymax: a.cy + yr / 2.0, ymin: a.cy - yr / 2.0,
            max_iter: a.iterations,
        };
        render(&mut buffer, params, &stretch(32, base_palette));
        write_file(&buffer, &a.output, a.width, a.height).unwrap();
    }
}
