pub mod stdin;
pub mod file;

use std::io::Error;
pub use crate::supply::stdin::StdinInput;
pub use crate::supply::file::FileInput;

pub enum ReadResult<'a> {
    Line(&'a String),   // a line with contents
    EOF,                // EOF, lines might come up next
    Finished            // processed everything, no more lines
}

pub trait LineSupplier {
    fn get_line(&mut self) -> Result<ReadResult, Error>;
}

pub fn process<T: LineSupplier>(supplier: &mut T) -> Result<(), Error> {
    loop {
        match supplier.get_line()? {
            ReadResult::Line(line) => println!("{}", line.trim_end()),
            ReadResult::EOF => {}
            ReadResult::Finished => break
        }
    }
    Ok(())
}
