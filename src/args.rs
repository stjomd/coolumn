use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Columnate lists. ")]
pub struct Arguments {
    #[arg(help = "The files to read from. If none are specified, input is read from stdin.")]
    pub files: Vec<std::path::PathBuf>
}
