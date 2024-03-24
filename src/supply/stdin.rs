use std::io::{BufRead, stdin};
use crate::errors::Error;
use crate::supply::{LineSupplier, Progress};

/// A struct that encapsulates all necessary data to read lines from stdin.
/// After reading, all lines of input are stored in memory.
pub struct StdinInput {
    lines: Vec<String>,
    index: usize
}

impl StdinInput {

    /// A constructor for this struct.
    /// # Returns
    /// An instance of this struct.
    pub fn new() -> Self {
        Self { lines: vec![], index: 0 }
    }

    /// Reads the input from stdin and stores the lines in memory.
    fn load(&mut self) {
        self.lines = stdin().lock().lines()
            .map(|line| line.unwrap())
            .collect();
    }

}

impl LineSupplier for StdinInput {
    fn get_line(&mut self) -> Result<Progress, Error> {
        if self.index == 0 {
            self.load();
        }
        self.index += 1;
        match self.lines.get(self.index - 1) {
            Some(line) => Ok(Progress::Line(line)),
            None => Ok(Progress::Done)
        }
    }
}
