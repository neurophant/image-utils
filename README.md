# Image Utils

Image processing utilities

[![crates.io](https://img.shields.io/crates/v/image-utils.svg)](https://crates.io/crates/image-utils)
[![Build Status](https://travis-ci.org/embali/image-utils.svg?branch=master)](https://travis-ci.org/embali/image-utils)


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


## Run tests

```bash
cargo test
```
