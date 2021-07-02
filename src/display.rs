use anyhow::Result;
use image::DynamicImage;
use once_cell::sync::Lazy;

use crate::{Cell, Ipynb, Output};

static TERMINAL_WIDTH: Lazy<usize> = Lazy::new(|| {
    let (width, _) = term_size::dimensions().unwrap_or_default();
    width as usize
});

pub fn display_ipynb(ipynb: &Ipynb) -> Result<()> {
    for i in 0..ipynb.cells.len() {
        let cell = &ipynb.cells[i];
        display_cell(cell)?;

        if i != ipynb.cells.len() - 1 {
            println!();
        }
    }
    Ok(())
}

fn display_cell(cell: &Cell) -> Result<()> {
    match &cell.cell_type[..] {
        "code" => display_code(cell)?,
        "markdown" => display_markdown(cell),
        "raw" => display_raw(cell),
        _ => return Ok(()),
    };
    Ok(())
}

fn display_code(cell: &Cell) -> Result<()> {
    println!(
        "[{}]:",
        if let Some(execution_count) = cell.execution_count {
            execution_count.to_string()
        } else {
            " ".to_string()
        }
    );

    print_with_terminal_width('=');
    display_source(&cell);
    display_output(&cell)?;
    print_with_terminal_width('=');

    Ok(())
}

fn display_markdown(cell: &Cell) {
    display_source(&cell);
}

fn display_raw(cell: &Cell) {
    print_with_terminal_width('=');
    display_source(&cell);
    print_with_terminal_width('=');
}

fn display_source(cell: &Cell) {
    println!("{}", cell.source.join(""));
}

fn display_output(cell: &Cell) -> Result<()> {
    let outputs = if let Some(outputs) = &cell.outputs {
        outputs
    } else {
        return Ok(());
    };

    if outputs.is_empty() {
        return Ok(());
    }

    print_with_terminal_width('·');

    for output in outputs.iter() {
        match &output.output_type[..] {
            "stream" => display_stream(output),
            "display_data" | "execute_result" => display_data(output)?,
            "error" => display_error(output),
            _ => continue,
        }
    }
    Ok(())
}

fn display_stream(output: &Output) {
    if let Some(text) = &output.text {
        println!("{}", text.join("").trim_end_matches('\n'));
    }
}

fn display_data(output: &Output) -> Result<()> {
    if let Some(data) = &output.data {
        if let Some(image_png) = &data.image_png {
            display_image_png(image_png)?;
        } else if let Some(text_plain) = &data.text_plain {
            println!("{}", text_plain.join("").trim_end_matches('\n'));
        }
    }
    Ok(())
}

fn display_error(output: &Output) {
    if let Some(traceback) = &output.traceback {
        println!("{}", traceback.join("").trim_end_matches('\n'));
    }
}

fn display_image_png(image_png: &str) -> Result<()> {
    let img = image::load_from_memory(&base64::decode(image_png.trim_end())?[..])?;

    let display_config = viuer::Config {
        transparent: true,
        absolute_offset: false,
        ..Default::default()
    };

    viuer::print(&DynamicImage::ImageRgb8(img.to_rgb8()), &display_config)?;
    Ok(())
}

fn print_with_terminal_width(c: char) {
    println!(
        "{}",
        std::iter::repeat(c)
            .take(*TERMINAL_WIDTH)
            .collect::<String>()
    );
}
