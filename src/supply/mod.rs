pub mod stdin;
pub mod file;

use crate::errors::Error;
pub use crate::supply::stdin::StdinInput;
pub use crate::supply::file::FileInput;

/// A collection of values to track progress of reading input.
pub enum Progress<'a> {
    /// Indicates that a read was successful and contains the contents of a line.
    Line(&'a str),
    /// Indicates that nothing was read (for example, EOF was reached), but valid lines might appear
    /// next.
    Continue,
    /// Indicates that all content has been read already.
    Done
}

// /// A collection of values to track progress of reading input.
// pub enum ProgressOwn {
//     /// Indicates that a read was successful and contains the contents of a line.
//     Line(String),
//     /// Indicates that all content has been read already.
//     Done
// }

pub trait LineSupplier {

    /// Returns a line of input.
    /// # Returns
    /// A result containing either a `Progress` value or an `Error`.
    /// ## Multiple files
    /// When reading from multiple files, it is possible to get a `Progress::EOF` value. That
    /// indicates an end of the current file, and a further call to this method can be made to
    /// retrieve a line from the next file. Once no more lines and files are available,
    /// `Progress::Done` is returned. The method `get_line_owned` abstracts this away.
    fn get_line(&mut self) -> Result<Progress, Error>;

    // /// Returns a line of input as an owned string.
    // /// This method utilizes `LineSupplier::get_line` and converts string slices to owned strings.
    // /// This allows to exclude the EOF case from the set of return values, but comes at a cost of
    // /// creating owned strings.
    // /// # Returns
    // /// A result containing either a `ProgressOwn` value or an `Error`.
    // fn get_line_owned(&mut self) -> Result<ProgressOwn, Error> {
    //     loop {
    //         match self.get_line()? {
    //             Progress::Line(line) => return Ok(ProgressOwn::Line(line.to_string())),
    //             Progress::EOF => continue,
    //             Progress::Done => return Ok(ProgressOwn::Done)
    //         }
    //     }
    // }

    /// Performs an action on each line of input.
    /// # Arguments
    /// - operation: a closure accepting a string slice (`&str`) and returning void.
    /// # Returns
    /// A `Result`; when successful, void; otherwise an error.
    fn for_each(&mut self, operation: fn(&str) -> ()) -> Result<(), Error> {
        loop {
            match self.get_line()? {
                Progress::Line(line) => operation(line.trim_end()),
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
}
