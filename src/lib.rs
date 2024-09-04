use std::{fs::File, io::{self, BufRead, BufReader}};
use anyhow::{Context, Result};




pub fn find_matches(content: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) -> Result<()>{    
    for line in content.lines() {
        match line {
            Ok(line) => { 
                if line.contains(pattern){
                    writeln!(writer, "{}", line).expect("can't read line");    
          }},
            Err(e) => {return Err(e.into());}
        };
       
}
Ok(())

}
  
  #[test]
  fn test_find_matches() {
    let mut result = Vec::new();
    // let file = std::fs::File::create(path)
    let con = BufReader::new(std::fs::File::open("text.txt").unwrap());
   let _ = find_matches(con, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
  }