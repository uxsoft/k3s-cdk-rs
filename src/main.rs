mod components;
mod templates;

use components::*;
use templates::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    ///https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

    let template = templates::DockerWeb::new("alpaca", "alpaca", "alpaca.uxsoft.cz");
    let yaml = template.to_yaml();
    println!("{yaml}");
}
