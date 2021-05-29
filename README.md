# machine-vision-formats

Type definitions for working with machine vision cameras.

This crate aims to be a lowest common denominator for working with images
from machine vision cameras from companies such as Basler, FLIR, and AVT.

- Can be compiled without standard library support (`no_std`).
- Includes strongly-typed pixel formats in the `pixel_format` module (e.g.
  `RGB8` and `Mono8`) to ensure correct API use.

Additionally several traits are defined to describe image data:

- `ImageData` defines the basics, such as image dimensions and the data
  buffer.
- `ImageMutData` is implemented for images with mutable data.
- `Stride` is implemented for images with strided data (i.e. each image row
   is encoded with exactly the same number of bytes, which may including
   padding).

This crate is used extensively in [Strand
Camera](https://github.com/strawlab/strand-braid).

## Potential further improvements

The list of pixel formats variants is currently limited rather limited. Please
submit an issue or, better, pull request for any additions needed.

We could also address the question of how endian-ness and packed-ness are
handled. Currently, these are not specified.

## See also

* [rust-cv/ndarray-image](https://github.com/rust-cv/ndarray-image)

## Test compilation with all feature variants

    cargo build
    cargo +nightly build --no-default-features --features "alloc"
    cargo +nightly build --no-default-features

## Code of conduct

Anyone who interacts with this software in any space, including but not limited
to this GitHub repository, must follow our [code of
conduct](code_of_conduct.md).

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
