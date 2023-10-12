use clap::Parser;
use mandelbrot_rs::color::stretch;
use mandelbrot_rs::color::XAOS;
use mandelbrot_rs::render;
use mandelbrot_rs::write_file;
use mandelbrot_rs::Params;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: String,
    #[arg(short, long, default_value_t = 1920)]
    width: usize,
    #[arg(short, long, default_value_t = 1080)]
    height: usize,
    #[arg(short, long, default_value_t = 512)]
    iterations: u32,
    #[arg(short, long, default_value_t = 1.0)]
    zoom: f64,
    #[arg(short, long, default_value_t = 0.0)]
    cx: f64,
    #[arg(short, long, default_value_t = 0.0)]
    cy: f64,
}

fn main() -> () {
    let args = Args::parse();

    let mut buffer = vec![0u32; args.width * args.height];
    let palette = stretch(8, &XAOS);
    let params = Params {
        width: args.width,
        height: args.height,
        xmax: 1.0,
        xmin: -2.5,
        ymax: 1.0,
        ymin: -1.0,
        max_iter: args.iterations,
    };
    render(&mut buffer, params, &palette);
    write_file(&buffer, &args.output, args.width, args.height).unwrap();
}
