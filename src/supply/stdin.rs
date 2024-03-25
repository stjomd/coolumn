use std::io::{BufRead, stdin};
use crate::errors::Error;
use crate::supply::{Line, LineSupplier, Progress};

/// A struct that encapsulates all necessary data to read lines from stdin.
/// After reading, all lines of input are stored in memory.
pub struct StdinInput {
    /// The lines of input.
    lines: Vec<String>,
    /// The index of the current line (that has not been returned yet).
    index: usize,
    /// Indicates whether the input lines have been stored in memory.
    loaded: bool
}

impl StdinInput {
    /// A constructor for this struct.
    /// # Returns
    /// An instance of this struct.
    pub fn new() -> Self {
        Self { lines: vec![], index: 0, loaded: false }
    }

    /// Reads the input from stdin and stores the lines in memory.
    fn load(&mut self) {
        self.lines = stdin().lock().lines()
            .map(|line| line.unwrap())
            .collect();
        self.loaded = true;
    }
}

impl LineSupplier for StdinInput {
    fn get_line(&mut self) -> Result<Progress, Error> {
        if !self.loaded {
            self.load();
        }
        self.index += 1;
        match self.lines.get(self.index - 1) {
            Some(line) => Ok(Progress::Line(Line::new(line))),
            None => Ok(Progress::Done)
        }
    }

    fn reset(&mut self) {
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::supply::Progress::{Done, Line};

    fn mock(lines: Vec<String>) -> StdinInput {
        let mut supplier = StdinInput::new();
        supplier.lines = lines;
        supplier.loaded = true;
        return supplier;
    }

    fn assert_case(expected_case: Progress, progress: Progress) -> () {
        assert_eq!(expected_case, progress, "Case does not match")
    }

    fn assert_line(expected_line: &str, progress: Progress) -> () {
        match progress {
            Line(line) => assert_eq!(expected_line, line.line, "Line does not match input"),
            case => panic!("Expected Progress::Line, got {:?}", case)
        }
    }

    #[test]
    fn should_return_line_and_done() -> Result<(), Error> {
        let lines = vec![
            String::from("line 1"),
            String::from(""),
            String::from("line 3"),
        ];
        let mut supplier = mock(lines.clone());
        assert_line(&lines[0], supplier.get_line()?);
        assert_line(&lines[1], supplier.get_line()?);
        assert_line(&lines[2], supplier.get_line()?);
        assert_case(Done, supplier.get_line()?);
        Ok(())
    }

    #[test]
    fn should_start_from_begin_after_reset() -> Result<(), Error> {
        let lines = vec![
            String::from("line 1"),
            String::from("line 2"),
        ];
        let mut supplier = mock(lines.clone());
        assert_line(&lines[0], supplier.get_line()?);
        supplier.reset();
        assert_line(&lines[0], supplier.get_line()?);
        assert_line(&lines[1], supplier.get_line()?);
        assert_case(Done, supplier.get_line()?);
        supplier.reset();
        assert_line(&lines[0], supplier.get_line()?);
        assert_line(&lines[1], supplier.get_line()?);
        assert_case(Done, supplier.get_line()?);
        Ok(())
    }
}