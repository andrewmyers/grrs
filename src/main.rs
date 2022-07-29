#![allow(unused)]

use std::env::args;
use std::path::PathBuf;
use clap::Parser;

// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,

    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: PathBuf
}

fn main() {
    let args = Cli::parse();

    println!("Pattern: {} Path: {:?}", args.pattern, args.path)
}
