use std::io::{BufRead, stdin};
use crate::errors::Error;
use crate::supply::{LineSupplier, Progress};

pub struct StdinInput {
    lines: Vec<String>,
    index: usize
}

impl StdinInput {
    pub fn new() -> Self {
        Self { lines: vec![], index: 0 }
    }
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
            Some(line) => Ok(Progress::Line(line.clone())),
            None => Ok(Progress::Finished)
        }
    }
}
