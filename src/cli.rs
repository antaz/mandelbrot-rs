use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub output: String,
    #[arg(short, long, default_value_t = 1920)]
    pub width: usize,
    #[arg(short, long, default_value_t = 1080)]
    pub height: usize,
    #[arg(short, long, default_value_t = 512)]
    pub iterations: u32,
    #[arg(short, long, default_value_t = 1.0)]
    pub zoom: f64,
    #[arg(short, long, default_value_t = -0.74364)]
    pub cx: f64,
    #[arg(short, long, default_value_t = 0.13182)]
    pub cy: f64,
    #[arg(short, long, default_value_t = 0)]
    pub frames: usize,
    #[arg(long, default_value_t = 1.05)]
    pub zoom_speed: f64,
    #[arg(long, default_value = "xaos")]
    pub palette: String,
}
