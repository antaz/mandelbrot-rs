use clap::Parser;
use mandelbrot_rs::color::stretch;
use mandelbrot_rs::color::XAOS;
use mandelbrot_rs::render;
use mandelbrot_rs::write_file;

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
}

fn main() -> () {
    let args = Args::parse();

    let mut buffer = vec![0u32; args.width * args.height];
    let palette = stretch(8, &XAOS);
    render(
        &mut buffer,
        args.width,
        args.height,
        args.iterations,
        &palette,
    );
    write_file(&buffer, &args.output, args.width, args.height).unwrap();
}
