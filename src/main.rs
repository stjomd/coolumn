mod args;
mod supply;

use crate::supply::{LineSupplier, FileInput, StdinInput};

fn main() {

    let args = args::argv();

    let mut supplier: Box<dyn LineSupplier>;
    if !args.files.is_empty() {
        supplier = Box::new(FileInput::new(args.files));
    } else {
        supplier = Box::new(StdinInput::new());
    }

    supplier
        .for_each(|line| println!("{line}"))
        .expect("Couldn't process");

}
