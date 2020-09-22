use clap::Clap;
use nbcat::{Cell, Ipynb, Opts};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let file = std::fs::read_to_string(&opts.input)?;
    let ipynb: Ipynb = serde_json::from_str(&file)?;
    for cell in ipynb.cells {
        display_source(&cell);
        println!("");
        display_output(&cell);
        println!("");
    }
    Ok(())
}

fn display_source(cell: &Cell) {
    println!(
        "[{}]:",
        if let Some(execution_count) = cell.execution_count {
            execution_count.to_string()
        } else {
            " ".to_string()
        }
    );
    println!("{}", cell.source.join(""));
}

fn display_output(cell: &Cell) {
    for output in cell.outputs.iter() {
        if let Some(text) = &output.text {
            println!("{}", text.join(""));
        } else if let Some(data) = &output.data {
            if let Some(text_plain) = &data.text_plain {
                println!("{}", text_plain.join(""));
            }
        }
    }
}
