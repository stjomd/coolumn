use crate::supply::LineSupplier;

mod args;
mod supply;

fn main() {

    let args = args::argv();

    let mut supplier: Box<dyn LineSupplier>;
    if !args.files.is_empty() {
        supplier = Box::new(supply::FileInput::new(args.files));
    } else {
        supplier = Box::new(supply::StdinInput::new());
    }
    supply::process(&mut supplier).expect("Couldn't process");

}
