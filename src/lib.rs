//! A crate to get images info and process them, including animated GIFs.
//!
//! # Example
//!
//! ```rust,ignore
//! extern crate image_utils;
//!
//! use std::path::Path;
//! use image_utils::{info, crop, resize};
//!
//! let path = Path::new("test.jpg");
//!
//! let inf = info(&path)?;
//! println!("{:?}", inf);
//!
//! crop(&path, 10, 10, 100, 100, &Path::new("cropped.jpg"))?;
//! resize(&path, 200, 200, &Path::new("resized.jpg"))?;
//!
//! ```

#![deny(missing_docs)]

extern crate image;
extern crate gif;
extern crate gif_dispose;

use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use image::{GenericImage, ImageFormat, ColorType, ImageRgba8, ImageBuffer, FilterType,
            guess_format};
use gif::{Decoder, SetParameter, ColorOutput, Frame, Encoder, Repeat};
use gif_dispose::Screen;

/// Common image information
#[derive(Debug, PartialEq)]
pub struct Info {
    /// Image format
    pub format: ImageFormat,
    /// Image color type
    pub color: ColorType,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Aspect ratio
    pub ratio: f32,
    /// Number of frames, can be greater than 1 for animated GIFs
    pub frames: u32,
}

/// Returns common information about image
///
/// `path` - image file to analyze
///
/// Returns Info struct
pub fn info(path: &Path) -> Result<Info, Box<Error>> {
    let mut fp = File::open(path)?;
    let mut buf = [0; 16];
    fp.read(&mut buf)?;
    let format = guess_format(&buf)?;

    let im = image::load(BufReader::new(File::open(path)?), format)?;
    let color = im.color();
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
        color: color,
        width: width,
        height: height,
        ratio: width as f32 / height as f32,
        frames: frames,
    })
}

/// Crops image, panics if passed coordinates or cropped image size are out of bounds of existing
/// image
///
/// `src` - source image file
///
/// `x` - width offset
///
/// `y` - height offset
///
/// `width` - crop width
///
/// `height` - crop height
///
/// `dest` - destination image file
pub fn crop(src: &Path,
            x: u32,
            y: u32,
            width: u32,
            height: u32,
            dest: &Path)
            -> Result<(), Box<Error>> {
    let inf = info(src)?;

    if x + width > inf.width || y + height > inf.height {
        panic!("out of existing image bounds");
    }

    match inf.format {
        ImageFormat::GIF => {
            let mut decoder = Decoder::new(File::open(src)?);
            decoder.set(ColorOutput::Indexed);
            let mut reader = decoder.read_info().unwrap();
            let mut screen = Screen::new(&reader);
            let mut result = File::create(dest)?;
            let mut encoder = Encoder::new(&mut result, width as u16, height as u16, &[])?;
            encoder.set(Repeat::Infinite)?;

            while let Some(frame) = reader.read_next_frame().unwrap() {
                screen.blit(&frame)?;
                let mut buf: Vec<u8> = Vec::new();
                for pixel in screen.pixels.iter() {
                    buf.push(pixel.r);
                    buf.push(pixel.g);
                    buf.push(pixel.b);
                    buf.push(pixel.a);
                }
                let mut im = ImageRgba8(ImageBuffer::from_raw(inf.width, inf.height, buf).unwrap());

                im = im.crop(x, y, width, height);
                let mut pixels = im.raw_pixels();
                let mut output = Frame::from_rgba(width as u16, height as u16, &mut *pixels);
                output.delay = frame.delay;
                encoder.write_frame(&output)?;
            }
        }
        _ => {
            let mut im = image::load(BufReader::new(File::open(src)?), inf.format)?;
            im = im.crop(x, y, width, height);
            let mut output = File::create(dest)?;
            im.save(&mut output, inf.format)?;
        }
    };

    Ok(())
}

/// Resizes image preserving its aspect ratio
///
/// `src` - source image file
///
/// `width` - max width
///
/// `height` - max height
///
/// `dest` - destination image file
pub fn resize(src: &Path, width: u32, height: u32, dest: &Path) -> Result<(), Box<Error>> {
    let inf = info(src)?;

    match inf.format {
        ImageFormat::GIF => {
            let mut decoder = Decoder::new(File::open(src)?);
            decoder.set(ColorOutput::Indexed);
            let mut reader = decoder.read_info().unwrap();
            let mut screen = Screen::new(&reader);
            let mut result = File::create(dest)?;

            let ratio = width as f32 / height as f32;
            let scale = if ratio > inf.ratio {
                height as f32 / inf.height as f32
            } else {
                width as f32 / inf.width as f32
            };
            let nwidth = (inf.width as f32 * scale) as u32;
            let nheight = (inf.height as f32 * scale) as u32;

            let mut encoder = Encoder::new(&mut result, nwidth as u16, nheight as u16, &[])?;
            encoder.set(Repeat::Infinite)?;

            while let Some(frame) = reader.read_next_frame().unwrap() {
                screen.blit(&frame)?;
                let mut buf: Vec<u8> = Vec::new();
                for pixel in screen.pixels.iter() {
                    buf.push(pixel.r);
                    buf.push(pixel.g);
                    buf.push(pixel.b);
                    buf.push(pixel.a);
                }
                let mut im = ImageRgba8(ImageBuffer::from_raw(inf.width, inf.height, buf).unwrap());

                im = im.resize(width, height, FilterType::Lanczos3);
                let mut pixels = im.raw_pixels();
                let mut output = Frame::from_rgba(nwidth as u16, nheight as u16, &mut *pixels);
                output.delay = frame.delay;
                encoder.write_frame(&output)?;
            }
        }
        _ => {
            let mut im = image::load(BufReader::new(File::open(src)?), inf.format)?;
            im = im.resize(width, height, FilterType::Lanczos3);
            let mut output = File::create(dest)?;
            im.save(&mut output, inf.format)?;
        }
    };

    Ok(())
}
