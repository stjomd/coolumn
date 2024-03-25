pub mod stdin;
pub mod file;

use lazy_static::lazy_static;
use regex::Regex;
use crate::errors::Error;
pub use crate::supply::stdin::StdinInput;
pub use crate::supply::file::FileInput;

lazy_static! {
    /// Regular expression that matches ANSI color sequences.
    static ref UNPRINTABLE_REGEX: Regex = Regex::new(r"\p{Cc}\[[0-9;]*[mK]").unwrap();
}

/// A struct that encapsulates the contents of a line and its clean version.
#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    /// The line read from input.
    pub line: &'a str,
    /// The line with unprintable characters removed.
    pub clean_line: String
}

impl<'a> Line<'a> {
    /// Constructs a new instance of this struct.
    /// # Arguments
    /// - line: a string slice.
    /// # Returns
    /// An instance of this struct that stores the passed in string slice.
    pub fn new(line: &'a str) -> Self {
        Line { line, clean_line: UNPRINTABLE_REGEX.replace_all(line, "").into_owned() }
    }
}

/// A collection of values to track progress of reading input.
#[derive(Debug, PartialEq)]
pub enum Progress<'a> {
    /// Indicates that a read was successful and contains the contents of a line.
    Line(Line<'a>),
    /// Indicates that nothing was read (for example, EOF was reached), but valid lines might appear
    /// next.
    Continue,
    /// Indicates that all content has been read already.
    Done
}

pub trait LineSupplier {

    /// Returns a line of input.
    /// # Returns
    /// A result containing either a `Progress` value or an `Error`.
    fn get_line(&mut self) -> Result<Progress, Error>;

    /// Resets the state of the supplier.
    /// After this call, supplier will be able to provide the input contents again.
    fn reset(&mut self);

    /// Performs an action on each line of input.
    /// # Arguments
    /// - operation: a closure accepting a string slice (`&str`) and returning void.
    /// # Returns
    /// A `Result`; when successful, void; otherwise an error.
    fn for_each(&mut self, operation: fn(&str) -> ()) -> Result<(), Error> {
        loop {
            match self.get_line()? {
                Progress::Line(line) => operation(line.line.trim_end()),
                Progress::Continue => continue,
                Progress::Done => return Ok(())
            }
        }
    }

}

impl LineSupplier for Box<dyn LineSupplier> {
    fn get_line(&mut self) -> Result<Progress, Error> {
        (**self).get_line()
    }
    fn reset(&mut self) {
        (**self).reset()
    }
}
