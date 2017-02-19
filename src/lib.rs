extern crate image;
extern crate gif;

use std::process::Command;
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

    Ok(Info {
        format: format,
        width: width,
        height: height,
        frames: frames,
    })
}


pub fn crop(src: &Path,
            x: u32,
            y: u32,
            width: u32,
            height: u32,
            dest: &Path)
            -> Result<bool, Box<Error>> {
    let inf = info(src)?;

    if x + width > inf.width || y + height > inf.height {
        panic!("out of existing image bounds");
    }

    let cmd = match inf.format {
        ImageFormat::GIF => {
            Command::new("convert").arg(src.to_str().unwrap())
                .arg("-coalesce")
                .arg("-repage")
                .arg("0x0")
                .arg("-crop")
                .arg(format!("{}x{}+{}+{}", width, height, x, y))
                .arg("+repage")
                .arg(dest.to_str().unwrap())
                .output()?
        }
        _ => {
            Command::new("convert").arg(src.to_str().unwrap())
                .arg("-crop")
                .arg(format!("{}x{}+{}+{}", width, height, x, y))
                .arg(dest.to_str().unwrap())
                .output()?
        }
    };

    Ok(cmd.status.success())
}
