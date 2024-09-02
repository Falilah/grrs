use clap::Parser;
use std::io::{self, BufReader, BufRead};
/// Search for a pattern in a file and display the lines that contain it.

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main() {    
    let args = Cli::parse();
    let f = std::fs::File::open(&args.path).expect("could not read file");
    let content = std::io::BufReader::new(f);

    for line in content.lines() {
        let line = line.expect("invalid line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }



}
