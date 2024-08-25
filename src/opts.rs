use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Opts {
    /// Input ipynb file path
    pub input: String,

    /// Do not display cell output
    #[arg(long)]
    pub ignore_output: bool,
}
