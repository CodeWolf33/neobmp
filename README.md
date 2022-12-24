NEOBMP
---
NeoBmp is a library to simplify operations with the `.bmp` file format.

This crate includes the following:

    - Structures to build your own BMP image.
    - A generic BmpImg struct to create them all automatically and group them.
    - Helpers to serialize all the structs AND the whole image [.to_bytes()]

Example usage:

```rust
use neobmp::BmpImg;

fn main() {
    let mut bmp_img = BmpImg::new(16, 16);

    bmp_img.fill_image(255, 0, 125);

    bmp_img.write_to_file("something.bmp");
}
```

Add this to your Cargo.toml
```toml
[dependencies]
neobmp = "0.1.5"
```