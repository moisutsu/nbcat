use clap::{crate_authors, crate_description, crate_version, Parser};

#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
pub struct Opts {
    #[clap(about = "Input ipynb file path")]
    pub input: String,
}
