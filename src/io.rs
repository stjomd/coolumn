use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;

pub fn read_file(path: &PathBuf) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
