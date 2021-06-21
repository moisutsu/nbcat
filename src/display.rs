use image::DynamicImage;

use crate::{Cell, Ipynb};

pub fn display_ipynb(ipynb: &Ipynb) {
    for cell in &ipynb.cells {
        display_cell(cell);
    }
}

fn display_cell(cell: &Cell) {
    display_source(&cell);
    println!();
    display_output(&cell);
    println!();
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

fn display_output(cell: &Cell) {
    println!(
        "Out: [{}]",
        if let Some(execution_count) = cell.execution_count {
            execution_count.to_string()
        } else {
            " ".to_string()
        }
    );
    for output in cell.outputs.iter() {
        if let Some(text) = &output.text {
            println!("{}", text.join(""));
        } else if let Some(data) = &output.data {
            if let Some(image_png) = &data.image_png {
                display_image_png(image_png);
            } else if let Some(text_plain) = &data.text_plain {
                println!("{}", text_plain.join(""));
            }
        }
    }
}

fn display_image_png(image_png: &str) {
    let img = image::load_from_memory(&base64::decode(image_png.trim_end()).unwrap()[..]).unwrap();

    let display_config = viuer::Config {
        transparent: true,
        absolute_offset: false,
        ..Default::default()
    };

    viuer::print(&DynamicImage::ImageRgb8(img.to_rgb8()), &display_config).unwrap();
}
