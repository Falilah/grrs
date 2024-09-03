use clap::Parser;
use std::io::{self, BufReader, BufRead};
use anyhow::{Context, Result};

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
    let f = std::fs::File::open(&args.path).with_context(|| format!("could not read file `{:?}`", &args.path))?;
    // print!("file:{:?}\n", f);
    let content = std::io::BufReader::new(f);
    for line in content.lines() {
      let line = match line {
          Ok(line) => {line},
          Err(e) => {return Err(e.into());}
      };
      println!("{}", line);
    }
    Ok(())

}
