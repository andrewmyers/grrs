#![allow(unused)]

use clap::Parser;
use std::env::args;
use std::path::PathBuf;
use std::fs;

// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,

    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let contents = fs::read_to_string(&args.path).expect("Could not read the file!");

    for line in contents.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
