use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Opts {
    #[clap(help = "Input ipynb file path")]
    pub input: String,
}
