mod macos_ls;

use crate::errors::Error;
use crate::supply::LineSupplier;

pub enum Printer {
    MacOsLs
}

pub fn print(supplier: &mut dyn LineSupplier, printer: Printer) -> Result<(), Error> {
    match printer {
        Printer::MacOsLs => macos_ls::print(supplier)
    }
}

