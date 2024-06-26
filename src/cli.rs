use std::path::Path;

use crate::dartfile;
use clap::Parser;

/// dartgun's command line interface

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    action: Option<String>,
}

fn fire() {
    println!("Attempting to find and read `dartfile.toml`.");
    let gun = dartfile::parse(Path::new("./dartgun.toml"));
    println!("Writing symlinks...");
    match gun.create_symlinks() {
        Ok(_) => println!("Symlinks created successfully!"),
        Err(err) => println!("Something went wrong while creating symlinks: {}", err),
    }
}

pub fn run_cli() {
    let cli = Cli::parse();
    match cli.action.as_deref() {
        Some("fire") => fire(),
        _ => println!("No action specified. Run `dartgun -h` for options."),
    };
}
