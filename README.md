# Image Utils

Image processing utilities

[![crates.io](https://img.shields.io/crates/v/image-utils.svg)](https://crates.io/crates/image-utils)
[![Build Status](https://travis-ci.org/embali/image-utils.svg?branch=master)](https://travis-ci.org/embali/image-utils)


## Documentation

https://docs.rs/image-utils/


## Functions

### Image information

```rust
extern crate image_utils;

use std::path::Path;
use image_utils::info;

fn main() {
    let inf = info(&Path::new("test.jpg")).unwrap();
    println!("{:?}", inf);
}
```

### Crop image

```rust
extern crate image_utils;

use std::path::Path;
use image_utils::crop;

fn main() {
    let success = crop(&Path::new("test.jpg"), 10, 10, 100, 100, &Path::new("cropped.jpg")).unwrap();
    println!("{:?}", success);
}
```

### Resize image

```rust
extern crate image_utils;

use std::path::Path;
use image_utils::resize;

fn main() {
    let success = resize(&Path::new("test.jpg"), 200, 200, &Path::new("resized.jpg")).unwrap();
    println!("{:?}", success);
}
```


## Run tests

```bash
cargo test
```
