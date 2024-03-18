mod args;
mod supply;

fn main() {

    let args = args::argv();

    if !args.files.is_empty() {
        let mut supplier = supply::FileInput::new(args.files);
        supply::process(&mut supplier).expect("Couldn't process");
    } else {
        let mut supplier = supply::StdinInput::new();
        supply::process(&mut supplier).expect("Couldn't process");
    }

}
