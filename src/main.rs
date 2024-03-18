mod args;
mod io;
mod supply;

fn main() {
    let args = args::argv();

    if !args.files.is_empty() {
        for file in args.files.iter() {
            let result = io::read_file(file);
            match result {
                Ok(_) => (),
                Err(error) => panic!("{:?}: {}", file, error)
            }
        }
    } else {
        let mut supplier = supply::StdinInput::new();
        supply::process(&mut supplier);
    }

}
