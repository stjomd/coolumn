mod args;
mod supply;
mod errors;
mod printer;

use crate::errors::EndResult;
use crate::printer::Printer;
use crate::supply::{LineSupplier, FileInput, StdinInput};

fn main() -> EndResult {

    let args = args::argv();

    let mut supplier: Box<dyn LineSupplier>;
    if !args.files.is_empty() {
        supplier = Box::new(FileInput::new(args.files));
    } else {
        supplier = Box::new(StdinInput::new());
    }

    let result = printer::print(&mut supplier, Printer::MacOsLs);
    EndResult(result)

}
