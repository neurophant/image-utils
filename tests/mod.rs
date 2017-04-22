#[cfg(test)]

extern crate image;
extern crate image_utils;

use std::fs::remove_file;
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
                   ratio: 510 as f32 / 350 as f32,
                   frames: 1,
               });
}

#[test]
fn test_info_jpg_tmp() {
    let inf = info(&Path::new("./tests/images/test.jpg.tmp")).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::JPEG,
                   color: ColorType::RGB(8),
                   width: 510,
                   height: 350,
                   ratio: 510 as f32 / 350 as f32,
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
                   ratio: 500 as f32 / 265 as f32,
                   frames: 12,
               });
}

#[test]
fn test_info_gif_tmp() {
    let inf = info(&Path::new("./tests/images/test.gif.tmp")).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::GIF,
                   color: ColorType::RGBA(8),
                   width: 500,
                   height: 265,
                   ratio: 500 as f32 / 265 as f32,
                   frames: 12,
               });
}

#[test]
fn test_crop_jpg() {
    let dest = Path::new("./tests/images/cropped.jpg");
    if dest.exists() {
        remove_file(&dest).unwrap();
    }
    crop(&Path::new("./tests/images/test.jpg"),
         10,
         10,
         100,
         100,
         &dest)
        .unwrap();
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::JPEG,
                   color: ColorType::RGB(8),
                   width: 100,
                   height: 100,
                   ratio: 100 as f32 / 100 as f32,
                   frames: 1,
               });
}

#[test]
fn test_crop_jpg_tmp() {
    let dest = Path::new("./tests/images/cropped.jpg.tmp");
    if dest.exists() {
        remove_file(&dest).unwrap();
    }
    crop(&Path::new("./tests/images/test.jpg.tmp"),
         10,
         10,
         100,
         100,
         &dest)
        .unwrap();
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::JPEG,
                   color: ColorType::RGB(8),
                   width: 100,
                   height: 100,
                   ratio: 100 as f32 / 100 as f32,
                   frames: 1,
               });
}

#[test]
fn test_crop_gif() {
    let dest = Path::new("./tests/images/cropped.gif");
    if dest.exists() {
        remove_file(&dest).unwrap();
    }
    crop(&Path::new("./tests/images/test.gif"),
         10,
         10,
         100,
         100,
         &dest)
        .unwrap();
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::GIF,
                   color: ColorType::RGBA(8),
                   width: 100,
                   height: 100,
                   ratio: 100 as f32 / 100 as f32,
                   frames: 12,
               });
}

#[test]
fn test_crop_gif_tmp() {
    let dest = Path::new("./tests/images/cropped.gif.tmp");
    if dest.exists() {
        remove_file(&dest).unwrap();
    }
    crop(&Path::new("./tests/images/test.gif.tmp"),
         10,
         10,
         100,
         100,
         &dest)
        .unwrap();
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::GIF,
                   color: ColorType::RGBA(8),
                   width: 100,
                   height: 100,
                   ratio: 100 as f32 / 100 as f32,
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
         &Path::new("./tests/images/cropped.jpg"))
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
         &Path::new("./tests/images/cropped.jpg"))
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
         &Path::new("./tests/images/cropped.gif"))
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
         &Path::new("./tests/images/cropped.gif"))
        .unwrap();
}

#[test]
fn test_resize_jpg() {
    let dest = Path::new("./tests/images/resized.jpg");
    if dest.exists() {
        remove_file(&dest).unwrap();
    }
    resize(&Path::new("./tests/images/test.jpg"), 200, 200, &dest).unwrap();
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::JPEG,
                   color: ColorType::RGB(8),
                   width: 200,
                   height: 137,
                   ratio: 200 as f32 / 137 as f32,
                   frames: 1,
               });
}

#[test]
fn test_resize_jpg_tmp() {
    let dest = Path::new("./tests/images/resized.jpg.tmp");
    if dest.exists() {
        remove_file(&dest).unwrap();
    }
    resize(&Path::new("./tests/images/test.jpg.tmp"), 200, 200, &dest).unwrap();
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::JPEG,
                   color: ColorType::RGB(8),
                   width: 200,
                   height: 137,
                   ratio: 200 as f32 / 137 as f32,
                   frames: 1,
               });
}

#[test]
fn test_resize_gif() {
    let dest = Path::new("./tests/images/resized.gif");
    if dest.exists() {
        remove_file(&dest).unwrap();
    }
    resize(&Path::new("./tests/images/test.gif"), 200, 200, &dest).unwrap();
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::GIF,
                   color: ColorType::RGBA(8),
                   width: 200,
                   height: 106,
                   ratio: 200 as f32 / 106 as f32,
                   frames: 12,
               });
}

#[test]
fn test_resize_gif_tmp() {
    let dest = Path::new("./tests/images/resized.gif.tmp");
    if dest.exists() {
        remove_file(&dest).unwrap();
    }
    resize(&Path::new("./tests/images/test.gif.tmp"), 200, 200, &dest).unwrap();
    let inf = info(&dest).unwrap();
    assert_eq!(inf,
               Info {
                   format: ImageFormat::GIF,
                   color: ColorType::RGBA(8),
                   width: 200,
                   height: 106,
                   ratio: 200 as f32 / 106 as f32,
                   frames: 12,
               });
}
