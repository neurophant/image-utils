#[cfg(test)]

extern crate image;
extern crate image_utils;

use std::path::Path;
use image::{ImageFormat, ColorType};
use image_utils::{info, Info, crop, resize};

#[test]
fn test_info_jpg() {
    let inf = info(&Path::new("./tests/images/test.jpg")).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::JPEG,
                   color: ColorType::RGB(8),
                   width: 510,
                   height: 350,
                   frames: 1,
               });
}

#[test]
fn test_info_gif() {
    let inf = info(&Path::new("./tests/images/test.gif")).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::GIF,
                   color: ColorType::RGBA(8),
                   width: 500,
                   height: 265,
                   frames: 12,
               });
}

#[test]
fn test_crop_jpg() {
    let dest = Path::new("./tests/images/cropped.jpg");
    assert!(crop(&Path::new("./tests/images/test.jpg"),
                 10,
                 10,
                 100,
                 100,
                 &dest,
                 10)
        .unwrap());
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::JPEG,
                   color: ColorType::RGB(8),
                   width: 100,
                   height: 100,
                   frames: 1,
               });
}

#[test]
fn test_crop_gif() {
    let dest = Path::new("./tests/images/cropped.gif");
    assert!(crop(&Path::new("./tests/images/test.gif"),
                 10,
                 10,
                 100,
                 100,
                 &dest,
                 10)
        .unwrap());
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::GIF,
                   color: ColorType::RGBA(8),
                   width: 100,
                   height: 100,
                   frames: 12,
               });
}

#[test]
#[should_panic]
fn test_crop_jpg_x() {
    crop(&Path::new("./tests/images/test.jpg"),
         10,
         10,
         1000,
         100,
         &Path::new("./tests/images/cropped.jpg"),
         10)
        .unwrap();
}

#[test]
#[should_panic]
fn test_crop_jpg_y() {
    crop(&Path::new("./tests/images/test.jpg"),
         10,
         10,
         100,
         1000,
         &Path::new("./tests/images/cropped.jpg"),
         10)
        .unwrap();
}

#[test]
#[should_panic]
fn test_crop_gif_x() {
    crop(&Path::new("./tests/images/test.gif"),
         10,
         10,
         1000,
         100,
         &Path::new("./tests/images/cropped.gif"),
         10)
        .unwrap();
}

#[test]
#[should_panic]
fn test_crop_gif_y() {
    crop(&Path::new("./tests/images/test.gif"),
         10,
         10,
         100,
         1000,
         &Path::new("./tests/images/cropped.gif"),
         10)
        .unwrap();
}

#[test]
fn test_resize_jpg() {
    let dest = Path::new("./tests/images/resized.jpg");
    assert!(resize(&Path::new("./tests/images/test.jpg"), 200, 200, &dest, 10).unwrap());
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::JPEG,
                   color: ColorType::RGB(8),
                   width: 200,
                   height: 137,
                   frames: 1,
               });
}

#[test]
fn test_resize_gif() {
    let dest = Path::new("./tests/images/resized.gif");
    assert!(resize(&Path::new("./tests/images/test.gif"), 200, 200, &dest, 10).unwrap());
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::GIF,
                   color: ColorType::RGBA(8),
                   width: 200,
                   height: 106,
                   frames: 12,
               });
}
