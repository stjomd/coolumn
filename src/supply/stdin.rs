use std::io::{BufRead, stdin};
use crate::errors::Error;
use crate::supply::{LineSupplier, Progress};

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
            Some(line) => Ok(Progress::Line(line)),
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

    #[test]
    fn should_return_line_and_done() -> Result<(), Error> {
        let lines = vec![
            String::from("line 1"),
            String::from(""),
            String::from("line 3"),
        ];
        let mut supplier = mock(lines.clone());
        assert_eq!(Line(&lines[0]), supplier.get_line()?, "Result of get_line does not match input");
        assert_eq!(Line(&lines[1]), supplier.get_line()?, "Result of get_line does not match input");
        assert_eq!(Line(&lines[2]), supplier.get_line()?, "Result of get_line does not match input");
        assert_eq!(Done, supplier.get_line()?, "Result of get_line is not `Done`");
        Ok(())
    }

    #[test]
    fn should_start_from_begin_after_reset() -> Result<(), Error> {
        let lines = vec![
            String::from("line 1"),
            String::from("line 2"),
        ];
        let mut supplier = mock(lines.clone());
        assert_eq!(Line(&lines[0]), supplier.get_line()?, "Result of get_line does not match input");
        supplier.reset();
        assert_eq!(Line(&lines[0]), supplier.get_line()?, "Result of get_line after reset is not the first line");
        assert_eq!(Line(&lines[1]), supplier.get_line()?, "Result of get_line does not match input");
        assert_eq!(Done, supplier.get_line()?, "Result of get_line is not `Done`");
        supplier.reset();
        assert_eq!(Line(&lines[0]), supplier.get_line()?, "Result of get_line after reset is not the first line");
        assert_eq!(Line(&lines[1]), supplier.get_line()?, "Result of get_line does not match input");
        assert_eq!(Done, supplier.get_line()?, "Result of get_line is not `Done`");
        Ok(())
    }
}