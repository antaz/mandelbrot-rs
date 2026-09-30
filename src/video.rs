use crate::color::{stretch, FIRE, XAOS};
use crate::{render, Params};
use std::io::Write;
use std::process::{Command, Stdio};

/// Render a zooming video sequence and pipe frames into ffmpeg
pub fn render_video(
    output: &str,
    width: usize,
    height: usize,
    max_iter: u32,
    zoom: f64,
    cx: f64,
    cy: f64,
    frames: usize,
    zoom_speed: f64,
    palette_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_palette = match palette_name.to_lowercase().as_str() {
        "fire" => &FIRE,
        _ => &XAOS,
    };
    let palette = stretch(32, base_palette);
    let mut child = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-f",
            "image2pipe",
            "-vcodec",
            "ppm",
            "-r",
            "30",
            "-i",
            "-",
            "-vcodec",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            output,
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().ok_or("Failed to open ffmpeg stdin")?;
    let mut buffer = vec![0u32; width * height];

    for i in 0..frames {
        let current_zoom = zoom * zoom_speed.powi(i as i32);
        let aspect = height as f64 / width as f64;
        let x_range = 3.5 / current_zoom;
        let y_range = x_range * aspect;
        let params = Params {
            width,
            height,
            xmax: cx + x_range / 2.0,
            xmin: cx - x_range / 2.0,
            ymax: cy + y_range / 2.0,
            ymin: cy - y_range / 2.0,
            max_iter,
        };
        render(&mut buffer, params, &palette);
        let header = format!("P6 {} {} 255\n", width, height);
        stdin.write_all(header.as_bytes())?;
        let ppm_bytes: Vec<u8> = buffer
            .iter()
            .map(|v| v.to_be_bytes()[1..4].iter().cloned().collect::<Vec<u8>>())
            .flatten()
            .collect();
        stdin.write_all(&ppm_bytes)?;
    }
    drop(stdin);
    child.wait()?;
    Ok(())
}
