use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::PathBuf;
use crate::supply::{LineSupplier, ReadResult};
use crate::supply::ReadResult::*;

pub struct FileInput {
    paths: Vec<PathBuf>,
    reader: Option<BufReader<File>>,
    line: String,
    counter: usize
}

impl FileInput {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self { paths, reader: None, line: "".to_string(), counter: 0 }
    }
    fn open(path: &PathBuf) -> Result<BufReader<File>, Error> {
        let file = File::open(path)?;
        Ok(BufReader::new(file))
    }
    fn open_next_file(&mut self) -> Result<(), Error> {
        self.reader = None;
        let path = self.paths.get(self.counter);
        if let Some(path) = path {
            self.reader = Some(Self::open(path)?);
            self.counter += 1;
        }
        Ok(())
    }
}

impl LineSupplier for FileInput {
    fn get_line(&mut self) -> Result<ReadResult, Error> {
        match self.reader {
            None => self.open_next_file()?,
            Some(_) => {}
        }
        if let Some(ref mut reader) = self.reader {
            self.line.clear();
            return match reader.read_line(&mut self.line)? {
                0 => { self.open_next_file()?; Ok(EOF) },
                _ => Ok(Line(&self.line))
            }
        }
        Ok(Finished)
    }
}
