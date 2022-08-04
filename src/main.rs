use anyhow::Ok;
use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self};
use std::io::{BufReader};
use std::path::PathBuf;

// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,

    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path).with_context(|| {
        format!(
            "Could not read file {}",
            args.path.to_owned().as_path().display()
        )
    })?;

    let file_buffer = BufReader::new(file);

    let stdout = io::stdout();
    let handle = io::BufWriter::new(stdout.lock());

    grrs::find_matches(file_buffer, &args.pattern, handle).unwrap();

    Ok(())
}