mod args;
mod io;

fn main() {
    let args = args::argv();

    if args.files.len() > 0 {
        for file in args.files.iter() {
            let result = io::read_file(file);
            match result {
                Ok(_) => (),
                Err(error) => panic!("{:?}: {}", file, error)
            }
        }
    } else {
        let lines = io::read_stdin();
        lines.iter().for_each(|line| println!("{}", line))
    }

}
