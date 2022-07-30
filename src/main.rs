use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::{ BufReader, BufRead};

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
    
    let f = File::open(args.path).expect("Could not read file");
    let f = BufReader::new(f);

    let mut line_num = 1;

    for line in f.lines() {
        let contents = line.unwrap();
        if contents.contains(&args.pattern) {
            println!("{}: {}", line_num, contents);
        }

        line_num += 1;
    }
}
