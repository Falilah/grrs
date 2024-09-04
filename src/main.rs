use anyhow::{Context, Result};
use clap::Parser;
use std::{fs::File, io::{self, BufRead, BufReader}};

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
    let f = std::fs::File::open(&args.path).with_context(|| format!("could not open file `{:?}`", &args.path))?;
    // print!("file:{:?}\n", f);
    let content = std::io::BufReader::new(f);
    for line in content.lines() {
      let line = match line {
          Ok(line) => {line},
          Err(e) => {return Err(e.into());}
      };
      if line.contains(&args.pattern){
        println!("{}", line);

      }
    }
    Ok(())

}

fn find_matches(content: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) -> Result<()>{
    
    for line in content.lines() {
        match line {
            Ok(line) => { 
                if line.contains(pattern){
                    writeln!(writer, "{}", line);    
          }},
            Err(e) => {return Err(e.into());}
        };
       
}
Ok(())

}
  
  #[test]
  fn test_find_matches() {
    let mut result = Vec::new();
    let con = BufReader::new(std::fs::File::open("text.txt").unwrap());
   let _ = find_matches(con, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
  }