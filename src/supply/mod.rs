pub mod stdin;
pub mod file;

use crate::errors::Error;
pub use crate::supply::stdin::StdinInput;
pub use crate::supply::file::FileInput;

/// A collection of values to track progress of reading input.
pub enum Progress {
    /// Indicates that a read was successful and contains the contents of a line.
    Line(String),
    /// Indicates that an EOF was reached when reading from a file.
    EOF,
    /// Indicates that all content has been read already.
    Finished
}

pub trait LineSupplier {
    fn get_line(&mut self) -> Result<Progress, Error>;
    fn for_each(&mut self, operation: fn(&str) -> ()) -> Result<(), Error> {
        loop {
            match self.get_line()? {
                Progress::Line(line) => operation(line.trim_end()),
                Progress::EOF => continue,
                Progress::Finished => break
            }
        }
        Ok(())
    }
}

impl LineSupplier for Box<dyn LineSupplier> {
    fn get_line(&mut self) -> Result<Progress, Error> {
        (**self).get_line()
    }
}
