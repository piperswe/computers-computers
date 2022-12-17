use std::{path::PathBuf, process::exit};

use clap::{Parser, Subcommand};
use simple_logger::SimpleLogger;

mod blogposts;
mod elements;

mod generate;
mod markdown;

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Generate a static site from the current site config
    Generate {
        /// The output path, where the static site should be written
        out_path: PathBuf,
    },
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let args = Args::parse();
    let err = match args.action {
        Action::Generate { out_path } => generate::generate(out_path),
    };
    if let Err(err) = err {
        println!("Unexpected error: {}", err);
        exit(1);
    }
}
