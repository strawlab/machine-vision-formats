use crate::{
    pixel_format::{Mono8, RGB8},
    ImageBuffer, ImageBufferRef, ImageData, Stride,
};
use std::ops::Deref;

// We should define a macro that will create these implementation for different
// types. Here there is a lot of code duplication for Luma and Rgb types, which
// is bad. Currently, this is a proof-of-concept to address this issue:
// https://github.com/strawlab/machine-vision-formats/issues/1

impl<C> ImageData<Mono8> for image::ImageBuffer<image::Luma<u8>, C>
where
    C: Deref<Target = [u8]> + AsRef<[u8]>,
{
    #[inline(always)]
    fn width(&self) -> u32 {
        image::ImageBuffer::width(self)
    }
    #[inline(always)]
    fn height(&self) -> u32 {
        image::ImageBuffer::height(self)
    }
    #[inline(always)]
    fn buffer_ref(&self) -> ImageBufferRef<'_, Mono8> {
        let data = self.as_flat_samples().samples;

        ImageBufferRef {
            pixel_format: std::marker::PhantomData,
            data,
        }
    }
    #[inline(always)]
    fn buffer(self) -> ImageBuffer<Mono8> {
        // TODO: can we check if somehow C is Vec<u8> and if so call `let data =
        // image::ImageBuffer::into_vec(self);`? That would let us eliminate a copy for that case.

        let data = self.as_flat_samples().samples.to_vec(); // copy the data

        ImageBuffer {
            pixel_format: std::marker::PhantomData,
            data,
        }
    }
}

impl<C> ImageData<RGB8> for image::ImageBuffer<image::Rgb<u8>, C>
where
    C: Deref<Target = [u8]> + AsRef<[u8]>,
{
    #[inline(always)]
    fn width(&self) -> u32 {
        image::ImageBuffer::width(self)
    }
    #[inline(always)]
    fn height(&self) -> u32 {
        image::ImageBuffer::height(self)
    }
    #[inline(always)]
    fn buffer_ref(&self) -> ImageBufferRef<'_, RGB8> {
        let data = self.as_flat_samples().samples;

        ImageBufferRef {
            pixel_format: std::marker::PhantomData,
            data,
        }
    }
    #[inline(always)]
    fn buffer(self) -> ImageBuffer<RGB8> {
        // TODO: can we check if somehow C is Vec<u8> and if so call `let data =
        // image::ImageBuffer::into_vec(self);`? That would let us eliminate a copy for that case.

        let data = self.as_flat_samples().samples.to_vec(); // copy the data

        ImageBuffer {
            pixel_format: std::marker::PhantomData,
            data,
        }
    }
}

impl<P, C> Stride for image::ImageBuffer<P, C>
where
    P: 'static + image::Pixel,
    C: Deref<Target = [<P as image::Pixel>::Subpixel]> + AsRef<[u8]>,
{
    #[inline(always)]
    fn stride(&self) -> usize {
        self.sample_layout().height_stride
    }
}

// // I don't see why this cannot work but probably requires implementing Into<Vec<u8>> on ImageBuffer.
// fn _test_image_buffer_implements_owned_image() {
//     // Compile-time test to ensure image::ImageBuffer can implement OwnedImage trait.
//     fn implements<T: crate::OwnedImage<Mono8>>() {}

//     implements::<image::ImageBuffer<image::Luma<u8>, Vec<u8>>>();
// }

#[test]
fn test_imagers_luma_to_mono8() {
    let imgx = 4;
    let imgy = 3;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let loc = y * 4 + x;
        let val = loc as u8;
        *pixel = image::Luma([val]);
    }

    // So our image is now:
    // 0  1  2  3
    // 4  5  6  7
    // 8  9 10 11

    assert_eq!(
        ImageData::image_data(&imgbuf),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    );

    assert_eq!(
        ImageData::buffer_ref(&imgbuf).data,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    );

    assert_eq!(Stride::stride(&imgbuf), 4);

    assert_eq!(
        ImageData::buffer(imgbuf).data,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    );
}

#[test]
fn test_imagers_rgb_to_rgb() {
    let imgx = 4;
    let imgy = 3;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let loc = y * 4 + x;
        let r = loc as u8;
        let b = r * 2;
        *pixel = image::Rgb([r, 0, b]);
    }

    // So red channel in our image is now:
    // 0  1  2  3
    // 4  5  6  7
    // 8  9 10 11

    assert_eq!(
        ImageData::image_data(&imgbuf),
        &[
            0, 0, 0, 1, 0, 2, 2, 0, 4, 3, 0, 6, 4, 0, 8, 5, 0, 10, 6, 0, 12, 7, 0, 14, 8, 0, 16, 9,
            0, 18, 10, 0, 20, 11, 0, 22
        ]
    );

    assert_eq!(
        ImageData::buffer_ref(&imgbuf).data,
        &[
            0, 0, 0, 1, 0, 2, 2, 0, 4, 3, 0, 6, 4, 0, 8, 5, 0, 10, 6, 0, 12, 7, 0, 14, 8, 0, 16, 9,
            0, 18, 10, 0, 20, 11, 0, 22
        ]
    );

    assert_eq!(Stride::stride(&imgbuf), 12);

    assert_eq!(
        ImageData::buffer(imgbuf).data,
        [
            0, 0, 0, 1, 0, 2, 2, 0, 4, 3, 0, 6, 4, 0, 8, 5, 0, 10, 6, 0, 12, 7, 0, 14, 8, 0, 16, 9,
            0, 18, 10, 0, 20, 11, 0, 22
        ]
    );
}
