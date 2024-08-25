use anyhow::Result;
use image::DynamicImage;

use crate::{Cell, Ipynb, Opts, Output};

pub struct DisplayIpynb {
    ipynb: Ipynb,
    opts: Opts,
    terminal_width: usize,
}

impl DisplayIpynb {
    pub fn new(ipynb: Ipynb, opts: Opts) -> DisplayIpynb {
        let (terminal_width, _) = term_size::dimensions().unwrap_or_default();

        DisplayIpynb {
            ipynb,
            opts,
            terminal_width,
        }
    }

    pub fn display(&self) -> Result<()> {
        self.display_ipynb(&self.ipynb)?;

        Ok(())
    }

    pub fn display_ipynb(&self, ipynb: &Ipynb) -> Result<()> {
        for i in 0..ipynb.cells.len() {
            let cell = &ipynb.cells[i];
            self.display_cell(cell)?;

            if i != ipynb.cells.len() - 1 {
                println!();
            }
        }
        Ok(())
    }

    fn display_cell(&self, cell: &Cell) -> Result<()> {
        match &cell.cell_type[..] {
            "code" => self.display_code(cell)?,
            "markdown" => self.display_markdown(cell),
            "raw" => self.display_raw(cell),
            _ => return Ok(()),
        };
        Ok(())
    }

    fn display_code(&self, cell: &Cell) -> Result<()> {
        println!(
            "[{}]:",
            if let Some(execution_count) = cell.execution_count {
                execution_count.to_string()
            } else {
                " ".to_string()
            }
        );

        self.print_with_terminal_width('=');
        self.display_source(cell);

        if !self.opts.ignore_output {
            self.display_output(cell)?;
        }

        self.print_with_terminal_width('=');

        Ok(())
    }

    fn display_markdown(&self, cell: &Cell) {
        self.display_source(cell);
    }

    fn display_raw(&self, cell: &Cell) {
        self.print_with_terminal_width('=');
        self.display_source(cell);
        self.print_with_terminal_width('=');
    }

    fn display_source(&self, cell: &Cell) {
        println!("{}", cell.source.join(""));
    }

    fn display_output(&self, cell: &Cell) -> Result<()> {
        let outputs = if let Some(outputs) = &cell.outputs {
            outputs
        } else {
            return Ok(());
        };

        if outputs.is_empty() {
            return Ok(());
        }

        self.print_with_terminal_width('·');

        for output in outputs.iter() {
            match &output.output_type[..] {
                "stream" => self.display_stream(output),
                "display_data" | "execute_result" => self.display_data(output)?,
                "error" => self.display_error(output),
                _ => continue,
            }
        }
        Ok(())
    }

    fn display_stream(&self, output: &Output) {
        if let Some(text) = &output.text {
            println!("{}", text.join("").trim_end_matches('\n'));
        }
    }

    fn display_data(&self, output: &Output) -> Result<()> {
        if let Some(data) = &output.data {
            if let Some(image_png) = &data.image_png {
                self.display_image_png(image_png)?;
            } else if let Some(text_plain) = &data.text_plain {
                println!("{}", text_plain.join("").trim_end_matches('\n'));
            }
        }
        Ok(())
    }

    fn display_error(&self, output: &Output) {
        if let Some(traceback) = &output.traceback {
            println!("{}", traceback.join("").trim_end_matches('\n'));
        }
    }

    fn display_image_png(&self, image_png: &str) -> Result<()> {
        let img = image::load_from_memory(&base64::decode(image_png.trim_end())?[..])?;

        let display_config = viuer::Config {
            transparent: true,
            absolute_offset: false,
            ..Default::default()
        };

        viuer::print(&DynamicImage::ImageRgb8(img.to_rgb8()), &display_config)?;
        Ok(())
    }

    fn print_with_terminal_width(&self, c: char) {
        println!(
            "{}",
            std::iter::repeat(c)
                .take(self.terminal_width)
                .collect::<String>()
        );
    }
}
