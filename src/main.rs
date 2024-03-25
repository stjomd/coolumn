mod args;
mod supply;
mod errors;
mod printer;

use crate::errors::EndResult;
use crate::printer::Printer;
use crate::supply::{LineSupplier, Source::*};

fn main() -> EndResult {

    let args = args::argv();

    let mut supplier: Box<dyn LineSupplier>;
    if args.files.is_empty() {
        supplier = supply::new(Stdin);
    } else {
        supplier = supply::new(Files(args.files));
    }

    let result = printer::print(&mut supplier, Printer::MacOsLs);
    EndResult(result)

}
