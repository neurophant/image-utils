#[cfg(test)]

extern crate image;
extern crate image_utils;

use std::path::Path;
use image::ImageFormat;
use image_utils::{info, Info};


#[test]
fn test_info_jpg() {
    let inf = info(&Path::new("./tests/test.jpg")).unwrap();
    assert_eq!(inf, Info {format: ImageFormat::JPEG, width: 510, height: 350, frames: 1});
}


#[test]
fn test_info_gif() {
    let inf = info(&Path::new("./tests/test.gif")).unwrap();
    assert_eq!(inf, Info {format: ImageFormat::GIF, width: 500, height: 265, frames: 12});
}
