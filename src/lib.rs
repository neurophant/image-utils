//! A crate to get images info and process them.
//!
//! Requires ImageMagick installed to function properly since some functions uses its command line
//! tools.
//!
//! # Example
//!
//! ```no_run
//! extern crate image_utils;
//!
//! use std::path::Path;
//! use image_utils::{info, crop, resize};
//!
//! let path = Path::new("test.jpg");
//!
//! let inf = info(&path)?;
//! let cropped = crop(&path, 10, 10, 100, 100, &Path::new("cropped.jpg"), 10)?;
//! let resized = resize(&path, 200, 200, &Path::new("resized.jpg"), 10)?;
//!
//! println!("{:?} {:?} {:?}", inf, cropped, resized);
//! ```

#![deny(missing_docs)]

extern crate image;
extern crate gif;
extern crate wait_timeout;

use std::process::Command;
use std::error::Error;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use image::{GenericImage, ImageFormat, guess_format};
use gif::Decoder;
use wait_timeout::ChildExt;

/// info function result
#[derive(Debug, PartialEq)]
pub struct Info {
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
    pub frames: u32,
}

/// Returns basic information about image
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

/// Crops image, panics if passed coordinates or cropped image size are out of bounds of existing image,
/// fails if timeout exceeded
pub fn crop(src: &Path,
            x: u32,
            y: u32,
            width: u32,
            height: u32,
            dest: &Path,
            timeout: u32)
            -> Result<bool, Box<Error>> {
    let inf = info(src)?;

    if x + width > inf.width || y + height > inf.height {
        panic!("out of existing image bounds");
    }

    let srcs = src.to_str().unwrap();
    let dests = dest.to_str().unwrap();
    let dims = format!("{}x{}+{}+{}", width, height, x, y);

    let mut child = match inf.format {
        ImageFormat::GIF => {
            Command::new("convert").arg(srcs)
                .arg("-coalesce")
                .arg("-repage")
                .arg("0x0")
                .arg("-crop")
                .arg(dims)
                .arg("+repage")
                .arg(dests)
                .spawn()?
        }
        _ => {
            Command::new("convert").arg(srcs)
                .arg("-crop")
                .arg(dims)
                .arg(dests)
                .spawn()?
        }
    };

    let success = match child.wait_timeout(Duration::from_secs(timeout as u64))? {
        Some(status) => status.success(),
        None => {
            child.kill()?;
            child.wait()?.success()
        }
    };

    Ok(success)
}

/// Resizes image preserving its aspect ratio, fails if timeout exceeded
pub fn resize(src: &Path,
              width: u32,
              height: u32,
              dest: &Path,
              timeout: u32)
              -> Result<bool, Box<Error>> {
    let inf = info(src)?;

    let mut srcs = src.to_str().unwrap();
    let dests = dest.to_str().unwrap();

    let duration = Duration::from_secs(timeout as u64);

    let temp = match inf.format {
        ImageFormat::GIF => {
            let mut child = Command::new("convert").arg(srcs)
                .arg("-coalesce")
                .arg(dests)
                .spawn()?;

            srcs = dests;

            match child.wait_timeout(duration)? {
                Some(status) => status.success(),
                None => {
                    child.kill()?;
                    child.wait()?.success()
                }
            }
        }
        _ => false,
    };

    let mut child = Command::new("convert").arg("-size")
        .arg(format!("{}x{}", inf.width, inf.height))
        .arg(srcs)
        .arg("-resize")
        .arg(format!("{}x{}", width, height))
        .arg(dests)
        .spawn()?;

    let success = match child.wait_timeout(duration)? {
        Some(status) => status.success(),
        None => {
            child.kill()?;
            child.wait()?.success()
        }
    };

    if temp && !success {
        fs::remove_file(dests)?;
    }

    Ok(success)
}
