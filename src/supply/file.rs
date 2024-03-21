use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use crate::errors::Error;
use crate::supply::{LineSupplier, Progress};
use crate::supply::Progress::*;

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
        let file = File::open(path);
        match file {
            Ok(file) => Ok(BufReader::new(file)),
            Err(error) => Err(Error::IO(path.display().to_string(), error))
        }
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
    fn get_line(&mut self) -> Result<Progress, Error> {
        if let None = self.reader {
            self.open_next_file()?
        }
        if let Some(ref mut reader) = self.reader {
            self.line.clear();
            return match reader.read_line(&mut self.line) {
                Ok(0) => { self.open_next_file()?; Ok(EOF) },
                Ok(_) => Ok(Line(self.line.clone())),
                Err(error) => Err(Error::IO(self.paths[self.counter].display().to_string(), error))
            };
        }
        Ok(Finished)
    }
}
