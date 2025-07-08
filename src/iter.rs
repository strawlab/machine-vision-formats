//! Types to facilitate iterating over images

use crate::{pixel_format, ImageMutStride, ImageStride, PixelFormat};

/// An image whose rows can be iterated over.
// In a semver-breaking change, we could eliminate this trait and make its
// method part of ImageStride.
pub trait HasRowChunksExact<F>: ImageStride<F> {
    fn rowchunks_exact(&self) -> RowChunksExact<'_>;
}

impl<S, F> HasRowChunksExact<F> for S
where
    S: ImageStride<F>,
    F: PixelFormat,
{
    fn rowchunks_exact(&self) -> RowChunksExact<'_> {
        let fmt = pixel_format::pixfmt::<F>().unwrap();
        let valid_stride = fmt.bits_per_pixel() as usize * self.width() as usize / 8;

        let stride = self.stride();
        let height = self.height() as usize;
        let buf = self.buffer_ref().data;
        let max_len = buf.len().min(stride * height);
        let buf = &buf[..max_len];

        RowChunksExact {
            buf,
            stride,
            valid_stride,
        }
    }
}

pub struct RowChunksExact<'a> {
    buf: &'a [u8],
    stride: usize,
    valid_stride: usize,
}

impl std::fmt::Debug for RowChunksExact<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RowChunksExact")
            .field("stride", &self.stride)
            .field("valid_stride", &self.valid_stride)
            .finish_non_exhaustive()
    }
}

impl<'a> Iterator for RowChunksExact<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() >= self.valid_stride {
            let mut data: &'a [u8] = &[];
            std::mem::swap(&mut self.buf, &mut data);
            if data.len() > self.stride {
                let (first, rest) = data.split_at(self.stride);
                self.buf = rest;
                Some(&first[..self.valid_stride])
            } else {
                Some(&data[..self.valid_stride])
            }
        } else {
            None
        }
    }
}

/// An image whose mutable rows can be iterated over.
// In a semver-breaking change, we could eliminate this trait and make its
// method part of ImageMutStride.
pub trait HasRowChunksExactMut<F>: ImageMutStride<F> {
    fn rowchunks_exact_mut(&mut self) -> RowChunksExactMut<'_>;
}
impl<S, F> HasRowChunksExactMut<F> for S
where
    S: ImageMutStride<F>,
    F: PixelFormat,
{
    fn rowchunks_exact_mut(&mut self) -> RowChunksExactMut<'_> {
        let fmt = pixel_format::pixfmt::<F>().unwrap();
        let valid_stride = fmt.bits_per_pixel() as usize * self.width() as usize / 8;

        let stride = self.stride();
        let height = self.height() as usize;
        let buf = self.buffer_mut_ref().data;
        let max_len = buf.len().min(stride * height);
        let buf = &mut buf[..max_len];
        RowChunksExactMut {
            buf,
            stride,
            valid_stride,
        }
    }
}

pub struct RowChunksExactMut<'a> {
    buf: &'a mut [u8],
    stride: usize,
    valid_stride: usize,
}

impl std::fmt::Debug for RowChunksExactMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RowChunksExactMut")
            .field("stride", &self.stride)
            .field("valid_stride", &self.valid_stride)
            .finish_non_exhaustive()
    }
}

impl<'a> Iterator for RowChunksExactMut<'a> {
    type Item = &'a mut [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() >= self.valid_stride {
            let mut data: &'a mut [u8] = &mut [];
            std::mem::swap(&mut self.buf, &mut data);
            if data.len() > self.stride {
                let (first, rest) = data.split_at_mut(self.stride);
                self.buf = rest;
                Some(&mut first[..self.valid_stride])
            } else {
                Some(&mut data[..self.valid_stride])
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        iter::{HasRowChunksExact, HasRowChunksExactMut},
        pixel_format::Mono8,
        ImageBuffer, ImageBufferMutRef, ImageBufferRef, ImageData, ImageMutData, Stride,
    };

    struct RoiIm<'a> {
        width: u32,
        height: u32,
        stride: usize,
        buf: &'a [u8],
    }

    impl Stride for RoiIm<'_> {
        fn stride(&self) -> usize {
            self.stride
        }
    }

    impl ImageData<Mono8> for RoiIm<'_> {
        fn width(&self) -> u32 {
            self.width
        }
        fn height(&self) -> u32 {
            self.height
        }
        fn buffer_ref(&self) -> ImageBufferRef<'_, Mono8> {
            ImageBufferRef {
                data: self.buf,
                pixel_format: std::marker::PhantomData,
            }
        }
        fn buffer(self) -> ImageBuffer<Mono8> {
            // copy the data
            self.buffer_ref().to_buffer()
        }
    }

    struct RoiImMut<'a> {
        width: u32,
        height: u32,
        stride: usize,
        buf: &'a mut [u8],
    }

    impl Stride for RoiImMut<'_> {
        fn stride(&self) -> usize {
            self.stride
        }
    }

    impl ImageData<Mono8> for RoiImMut<'_> {
        fn width(&self) -> u32 {
            self.width
        }
        fn height(&self) -> u32 {
            self.height
        }
        fn buffer_ref(&self) -> ImageBufferRef<'_, Mono8> {
            ImageBufferRef {
                data: self.buf,
                pixel_format: std::marker::PhantomData,
            }
        }
        fn buffer(self) -> ImageBuffer<Mono8> {
            // copy the data
            self.buffer_ref().to_buffer()
        }
    }

    impl ImageMutData<Mono8> for RoiImMut<'_> {
        fn buffer_mut_ref(&mut self) -> ImageBufferMutRef<'_, Mono8> {
            ImageBufferMutRef {
                data: self.buf,
                pixel_format: std::marker::PhantomData,
            }
        }
    }

    #[test]
    fn test_roi_at_start() {
        const STRIDE: usize = 10;
        const ORIG_W: usize = 10;
        const ORIG_H: usize = 10;
        let mut image_data = [0u8; STRIDE * ORIG_H];

        // fill with useful pattern
        for row in 0..ORIG_H {
            for col in 0..ORIG_W {
                image_data[row * STRIDE + col] = (row * 10_usize + col) as u8;
            }
        }

        // generate an ROI
        let width = 2;
        let height = 2;
        let (row, col) = (2, 2);

        // create image of this ROI
        let im = RoiIm {
            width,
            height,
            stride: STRIDE,
            buf: &image_data[(row * STRIDE + col)..],
        };

        let mut rowchunk_iter = im.rowchunks_exact();
        assert_eq!(rowchunk_iter.next(), Some(&[22, 23][..]));
        assert_eq!(rowchunk_iter.next(), Some(&[32, 33][..]));
        assert_eq!(rowchunk_iter.next(), None);
    }

    #[test]
    fn test_roi_at_end() {
        const STRIDE: usize = 10;
        const ORIG_W: usize = 10;
        const ORIG_H: usize = 10;
        let mut image_data = [0u8; STRIDE * ORIG_H];

        // fill with useful pattern
        for row in 0..ORIG_H {
            for col in 0..ORIG_W {
                image_data[row * STRIDE + col] = (row * 10_usize + col) as u8;
            }
        }

        // generate an ROI
        let width = 3;
        let height = 4;
        let (row, col) = (6, 7);

        // create image of this ROI
        let im = RoiIm {
            width,
            height,
            stride: STRIDE,
            buf: &image_data[(row * STRIDE + col)..],
        };

        let mut rowchunk_iter = im.rowchunks_exact();
        assert_eq!(rowchunk_iter.next(), Some(&[67, 68, 69][..]));
        assert_eq!(rowchunk_iter.next(), Some(&[77, 78, 79][..]));
        assert_eq!(rowchunk_iter.next(), Some(&[87, 88, 89][..]));
        assert_eq!(rowchunk_iter.next(), Some(&[97, 98, 99][..]));
        assert_eq!(rowchunk_iter.next(), None);
    }

    #[test]
    fn test_mut_roi_at_start() {
        const STRIDE: usize = 10;
        const ORIG_W: usize = 10;
        const ORIG_H: usize = 10;
        let mut image_data = [0u8; STRIDE * ORIG_H];

        // fill with useful pattern
        for row in 0..ORIG_H {
            for col in 0..ORIG_W {
                image_data[row * STRIDE + col] = (row * 10_usize + col) as u8;
            }
        }

        // generate an ROI
        let width = 2;
        let height = 2;
        let (row, col) = (2, 2);

        {
            // create image of this ROI
            let mut im = RoiImMut {
                width,
                height,
                stride: STRIDE,
                buf: &mut image_data[(row * STRIDE + col)..],
            };

            let mut rowchunk_iter = im.rowchunks_exact_mut();
            let mut row2 = rowchunk_iter.next();
            assert_eq!(row2, Some(&mut [22, 23][..]));
            row2.as_mut().unwrap()[0] += 100;
            row2.as_mut().unwrap()[1] += 100;
            let mut row3 = rowchunk_iter.next();
            assert_eq!(row3, Some(&mut [32, 33][..]));
            row3.as_mut().unwrap()[0] += 100;
            row3.as_mut().unwrap()[1] += 100;
            assert_eq!(rowchunk_iter.next(), None);
        }

        // create image of this ROI
        let im = RoiIm {
            width,
            height,
            stride: STRIDE,
            buf: &image_data[(row * STRIDE + col)..],
        };

        let mut rowchunk_iter = im.rowchunks_exact();
        assert_eq!(rowchunk_iter.next(), Some(&[122, 123][..]));
        assert_eq!(rowchunk_iter.next(), Some(&[132, 133][..]));
        assert_eq!(rowchunk_iter.next(), None);
    }

    #[test]
    fn test_mut_roi_at_end() {
        const STRIDE: usize = 10;
        const ORIG_W: usize = 10;
        const ORIG_H: usize = 10;
        let mut image_data = [0u8; STRIDE * ORIG_H];

        // fill with useful pattern
        for row in 0..ORIG_H {
            for col in 0..ORIG_W {
                image_data[row * STRIDE + col] = (row * 10_usize + col) as u8;
            }
        }

        // generate an ROI
        let width = 3;
        let height = 4;
        let (row, col) = (6, 7);

        {
            // create image of this ROI
            let mut im = RoiImMut {
                width,
                height,
                stride: STRIDE,
                buf: &mut image_data[(row * STRIDE + col)..],
            };

            let mut rowchunk_iter = im.rowchunks_exact_mut();
            for row_num in row..(row + height as usize) {
                let mut this_row = rowchunk_iter.next();
                assert_eq!(
                    this_row,
                    Some(
                        &mut [
                            row_num as u8 * 10 + col as u8,
                            row_num as u8 * 10 + col as u8 + 1,
                            row_num as u8 * 10 + col as u8 + 2
                        ][..]
                    )
                );
                for col in 0..width as usize {
                    this_row.as_mut().unwrap()[col] += 100;
                }
            }
            assert_eq!(rowchunk_iter.next(), None);
        }

        // create image of this ROI
        let im = RoiIm {
            width,
            height,
            stride: STRIDE,
            buf: &image_data[(row * STRIDE + col)..],
        };

        let mut rowchunk_iter = im.rowchunks_exact();
        assert_eq!(rowchunk_iter.next(), Some(&[167, 168, 169][..]));
        assert_eq!(rowchunk_iter.next(), Some(&[177, 178, 179][..]));
        assert_eq!(rowchunk_iter.next(), Some(&[187, 188, 189][..]));
        assert_eq!(rowchunk_iter.next(), Some(&[197, 198, 199][..]));
        assert_eq!(rowchunk_iter.next(), None);
    }
}
