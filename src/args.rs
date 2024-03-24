use clap::Parser;

/// A struct containing the definitions of arguments to this program.
#[derive(Parser, Debug)]
#[command(version, about = "Columnate lists. ")]
pub struct Arguments {
    #[arg(help = "The files to read from. If none are specified, input is read from stdin.")]
    pub files: Vec<std::path::PathBuf>
}

/// Parses the arguments to this program and returns an `Arguments` instance.
pub fn argv() -> Arguments {
    Arguments::parse()
}
