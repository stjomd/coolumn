use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use crate::errors::Error;
use crate::supply::{LineSupplier, Progress};
use crate::supply::Progress::*;

/// A struct that encapsulates necessary data to read from files.
/// Only one line of input at a time is stored in memory.
pub struct FileInput {
    /// Paths of input files.
    paths: Vec<PathBuf>,
    /// The current reader for reading contents from the current file.
    reader: Option<BufReader<File>>,
    /// The last successfully read line.
    line: String,
    /// A counter pointing to the current path in `paths`.
    counter: usize
}

impl FileInput {

    /// A constructor for this struct.
    /// # Arguments
    /// - paths: a vector of input paths.
    /// # Returns
    /// An instance of this struct.
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self { paths, reader: None, line: "".to_string(), counter: 0 }
    }

    /// Opens a file at the specified path.
    /// # Arguments
    /// - path: a path to the file.
    /// # Returns
    /// A `Result`, which, when successful, contains a reader for the specified file. Otherwise,
    /// an error is returned.
    fn open(path: &PathBuf) -> Result<BufReader<File>, Error> {
        let file = File::open(path);
        match file {
            Ok(file) => Ok(BufReader::new(file)),
            Err(error) => Err(Error::IO(path.display().to_string(), error))
        }
    }

    /// Opens the next file, specified by the path in `self.paths` at index `self.counter`.
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
                Ok(0) => { self.open_next_file()?; Ok(Continue) },
                Ok(_) => Ok(Line(&self.line)),
                Err(error) => Err(Error::IO(self.paths[self.counter].display().to_string(), error))
            };
        }
        Ok(Done)
    }

    fn reset(&mut self) {
        self.reader = None;
        self.line = "".to_string();
        self.counter = 0;
    }

}
