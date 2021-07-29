use crate::{pixel_format::Mono8, ImageBuffer, ImageBufferRef, ImageData, Stride};
use std::ops::Deref;

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

impl<C> Stride for image::ImageBuffer<image::Luma<u8>, C>
where
    C: Deref<Target = [u8]> + AsRef<[u8]>,
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
