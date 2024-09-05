use clap::Parser;
use anyhow::{Context, Result};

// use std::{fs::File, io::{self, BufRead, BufReader}};

/// Search for a pattern in a file and display the lines that contain it.

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main() -> Result<()> {    
    let args = Cli::parse();
    if args.pattern.is_empty(){
        return Err(anyhow::anyhow!("empty pattern to search"));

    }
    let f = std::fs::File::open(&args.path).with_context(|| format!("could not open file `{:?}`", &args.path))?;

    let content = std::io::BufReader::new(f);
    let _ = grrs::find_matches(content, &args.pattern, std::io::stdout());
    Ok(())

}

