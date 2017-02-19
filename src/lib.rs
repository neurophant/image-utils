extern crate image;
extern crate gif;

use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use image::{GenericImage, ImageFormat, guess_format};
use gif::Decoder;


#[derive(Debug, PartialEq)]
pub struct Info {
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
    pub frames: u32,
}


pub fn info(path: &Path) -> Result<Info, Box<Error>> {
    let mut im = File::open(path)?;
    let mut buf = [0; 16];
    im.read(&mut buf)?;
    let format = guess_format(&buf)?;

    let im = image::open(path)?;
    let (width, height) = im.dimensions();

    let frames = match format {
        ImageFormat::GIF => {
            let decoder = Decoder::new(File::open(path)?);
            let mut reader = decoder.read_info().unwrap();
            let mut frames = 0;
            while let Some(_) = reader.next_frame_info().unwrap() {
                frames += 1;
            }
            frames
        }
        _ => 1,
    };

    Ok(Info {format: format, width: width, height: height, frames: frames})
}
