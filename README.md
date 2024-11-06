# machine-vision-formats

[![Documentation](https://docs.rs/machine-vision-formats/badge.svg)](https://docs.rs/machine-vision-formats/)
[![Crates.io](https://img.shields.io/crates/v/machine-vision-formats.svg)](https://crates.io/crates/machine-vision-formats)

Types and traits for working with raw image data from machine vision cameras.

This crate aims to be a lowest common denominator for working with images
from machine vision cameras from companies such as Basler, FLIR, and AVT.

- Can be compiled without standard library support (`no_std`).
- Includes strongly-typed pixel formats in the `pixel_format` module (e.g.
  `RGB8` and `Mono8`) to ensure correct API use.
- Includes types to efficiently iterate through images respecting strided
  layouts in the [iter] module.
- Includes structs which reference image data in the [image_ref] module.
- Includes struct which owns image data in the [owned] module.

This crate is used extensively in [Strand
Camera](https://github.com/strawlab/strand-braid).

## Potential further improvements

The list of pixel formats variants is currently limited rather limited. Please
submit an issue or, better, pull request for any additions needed.

We could also address the question of how endian-ness and packed-ness are
handled. Currently, these are not specified.

We should investigate [`rgb`](https://crates.io/crates/rgb) and [`imgref`](https://crates.io/crates/imgref)
and see if this crate is completely redundant with those.

## See also

- [https://github.com/rust-cv/nshare](https://github.com/rust-cv/nshare)

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

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)
