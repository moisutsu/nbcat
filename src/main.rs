use anyhow::Result;
use clap::Clap;
use nbcat::{display_ipynb, Opts};

fn main() -> Result<()> {
    let opts = Opts::parse();

    let file = std::fs::read_to_string(&opts.input)?;
    let ipynb = serde_json::from_str(&file)?;
    display_ipynb(&ipynb)?;
    Ok(())
}
