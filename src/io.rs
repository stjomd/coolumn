use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Result};
use std::path::PathBuf;

pub fn read_stdin() -> Vec<String> {
    stdin().lock().lines().map(|line| line.unwrap()).collect()
}

pub fn read_file(path: &PathBuf) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
