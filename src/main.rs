mod components;
mod templates;

use clap::{Parser, Subcommand};
use components::*;
use std::process::Command;
use templates::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GetContexts,
    UseContext { name: String },
    Generate,
    Apply,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GetContexts => {
            get_contexts();
        }
        Commands::UseContext { name } => {
            use_context(name);
        }
        Commands::Generate => {
            generate();
        }
        Commands::Apply => {
            println!("myapp apply was used");
        }
    }
}

fn get_contexts() {
    let output = Command::new("kubectl")
        .args(["config", "get-contexts"])
        .output()
        .expect("failed to execute process");

    let output_str = std::str::from_utf8(&output.stdout).expect("failed to parse stdout");

    println!("{}", output_str);
}

fn use_context(name: &String) {
    let output = Command::new("kubectl")
        .args(["config", "use-context", &name])
        .output()
        .expect("failed to execute process");

    let output_str = std::str::from_utf8(&output.stdout).expect("failed to parse stdout");

    println!("{}", output_str);
}

fn generate() {
    let template = templates::DockerWeb::new("alpaca", "alpaca", "alpaca.uxsoft.cz");
    let yaml = template.to_yaml();
    println!("{yaml}");
}
