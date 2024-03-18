mod args;
mod supply;

use crate::supply::*;

fn main() {

    let args = args::argv();

    let mut supplier: Box<dyn LineSupplier>;
    if !args.files.is_empty() {
        supplier = Box::new(FileInput::new(args.files));
    } else {
        supplier = Box::new(StdinInput::new());
    }

    process(&mut supplier).expect("Couldn't process");

}
