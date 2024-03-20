use std::fmt::{Debug, Display, Formatter};
use std::process::{ExitCode, Termination};


/// A custom error type for this project.
#[derive(Debug)]
pub enum Error {
    IO(String, std::io::Error)
}


// Implement custom error messages here.
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(path, error) => write!(f, "{path}: {error}")
        }
    }
}
impl std::error::Error for Error { }


/// Wrapper for `Result<(), errors::Error>`.
///
/// Its implementation of the trait `Termination` allows for a more user-friendly error logging.
pub struct EndResult(pub Result<(), Error>);
impl Termination for EndResult {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(_) => ExitCode::SUCCESS,
            Err(error) => { eprintln!("{error}"); ExitCode::FAILURE }
        }
    }
}