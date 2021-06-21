use anyhow::Result;
use clap::Clap;
use nbcat::{display_ipynb, Ipynb, Opts};

fn main() -> Result<()> {
    let opts = Opts::parse();

    let file = std::fs::read_to_string(&opts.input)?;
    let ipynb: Ipynb = serde_json::from_str(&file)?;
    display_ipynb(&ipynb)?;
    Ok(())
}
