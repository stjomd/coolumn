pub mod stdin;
pub mod file;

use crate::errors::Error;
pub use crate::supply::stdin::StdinInput;
pub use crate::supply::file::FileInput;

pub enum Progress<'a> {
    Line(&'a str),      // a line with contents
    EOF,                // EOF, lines might come up next
    Finished            // processed everything, no more lines
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
