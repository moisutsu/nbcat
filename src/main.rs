use anyhow::Result;
use clap::Parser;
use nbcat::{DisplayIpynb, Opts};

fn main() -> Result<()> {
    let opts = Opts::parse();

    let file = std::fs::read_to_string(&opts.input)?;
    let ipynb = serde_json::from_str(&file)?;

    let display_ipynb = DisplayIpynb::new(ipynb, opts);
    display_ipynb.display()?;

    Ok(())
}
