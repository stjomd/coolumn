mod args;
mod supply;
mod errors;
mod format;

use crate::errors::EndResult;
use crate::supply::{LineSupplier, FileInput, StdinInput};

fn main() -> EndResult {

    let args = args::argv();

    let mut supplier: Box<dyn LineSupplier>;
    if !args.files.is_empty() {
        supplier = Box::new(FileInput::new(args.files));
    } else {
        supplier = Box::new(StdinInput::new());
    }

    let result = format::print(&mut supplier);
    EndResult(result)

}
