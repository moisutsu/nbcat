use anyhow::Result;
use image::DynamicImage;
use once_cell::sync::Lazy;
use terminal_size::{terminal_size, Height, Width};

use crate::{Cell, Ipynb, Output};

static TERMINAL_WIDTH: Lazy<usize> = Lazy::new(|| {
    let (Width(width), _) = terminal_size().unwrap_or((Width(0), Height(0)));
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
    println!(
        "{}",
        std::iter::repeat("=")
            .take(*TERMINAL_WIDTH)
            .collect::<String>()
    );
    display_source(&cell);

    display_output(&cell)?;
    println!(
        "{}",
        std::iter::repeat("=")
            .take(*TERMINAL_WIDTH)
            .collect::<String>()
    );
    Ok(())
}

fn display_source(cell: &Cell) {
    println!(
        "In: [{}]",
        if let Some(execution_count) = cell.execution_count {
            execution_count.to_string()
        } else {
            " ".to_string()
        }
    );
    println!("{}", cell.source.join(""));
}

fn display_output(cell: &Cell) -> Result<()> {
    if cell.outputs.is_empty() {
        return Ok(());
    }

    if let Some(execution_count) = cell.execution_count {
        println!(
            "{}",
            std::iter::repeat("·")
                .take(*TERMINAL_WIDTH)
                .collect::<String>()
        );
        println!("Out: [{}]", execution_count);
    } else {
        return Ok(());
    }

    for output in cell.outputs.iter() {
        match &output.output_type[..] {
            "stream" => display_stream(output),
            "display_data" => display_data(output)?,
            "error" => display_error(output),
            _ => continue,
        }
    }
    Ok(())
}

fn display_stream(output: &Output) {
    if let Some(text) = &output.text {
        println!("{}", text.join(""));
    }
}

fn display_data(output: &Output) -> Result<()> {
    if let Some(data) = &output.data {
        if let Some(image_png) = &data.image_png {
            display_image_png(image_png)?;
        } else if let Some(text_plain) = &data.text_plain {
            println!("{}", text_plain.join(""));
        }
    }
    Ok(())
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

fn display_error(output: &Output) {
    if let Some(traceback) = &output.traceback {
        println!("{}", traceback.join(""));
    }
}
