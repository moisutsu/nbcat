use clap::Clap;
use nbcat::{display_cell, Ipynb, Opts};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let file = std::fs::read_to_string(&opts.input)?;
    let ipynb: Ipynb = serde_json::from_str(&file)?;
    for cell in ipynb.cells {
        display_cell(&cell);
    }
    Ok(())
}
