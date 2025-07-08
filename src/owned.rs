//! Implementation of an image type which owns its own image buffer

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::{
    ImageBuffer, ImageBufferMutRef, ImageBufferRef, ImageData, ImageMutData, OwnedImageStride,
    PixelFormat, Stride,
};

// -----

/// An owned image buffer with strided pixel format `FMT`.
#[derive(Clone)]
pub struct OImage<FMT: PixelFormat> {
    buf: Vec<u8>,
    width: u32,
    height: u32,
    stride: usize,
    fmt: std::marker::PhantomData<FMT>,
}

impl<FMT: PixelFormat> ImageData<FMT> for OImage<FMT> {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn buffer_ref(&self) -> ImageBufferRef<'_, FMT> {
        ImageBufferRef::new(&self.buf)
    }
    fn buffer(self) -> ImageBuffer<FMT> {
        // move the data
        ImageBuffer {
            data: self.buf,
            pixel_format: self.fmt,
        }
    }
}

impl<FMT: PixelFormat> ImageMutData<FMT> for OImage<FMT> {
    fn buffer_mut_ref(&mut self) -> ImageBufferMutRef<'_, FMT> {
        ImageBufferMutRef::new(&mut self.buf)
    }
}

impl<FMT: PixelFormat> Stride for OImage<FMT> {
    fn stride(&self) -> usize {
        self.stride
    }
}

impl<FMT: PixelFormat> OImage<FMT> {
    /// Move a `Vec<u8>` buffer as the backing store for an ImageStruct for
    /// image.
    ///
    /// Returns None if the buffer is not large enough to store an image of the
    /// desired properties.
    pub fn new(width: u32, height: u32, stride: usize, buf: Vec<u8>) -> Option<Self> {
        let fmt = crate::pixel_format::pixfmt::<FMT>().unwrap();
        let min_stride = fmt.bits_per_pixel() as usize * width as usize / 8;

        if height > 0 {
            // Check buffer size. (With height==0, we accept zero length
            // buffer.)
            let sz = stride * (height as usize - 1) + min_stride;
            if buf.len() < sz {
                return None;
            }
        }

        Some(Self {
            width,
            height,
            stride,
            buf,
            fmt: std::marker::PhantomData,
        })
    }

    /// Allocate minimum size buffer for image and fill with zeros
    pub fn zeros(width: u32, height: u32, stride: usize) -> Option<Self> {
        let fmt = crate::pixel_format::pixfmt::<FMT>().unwrap();
        let valid_stride = fmt.bits_per_pixel() as usize * width as usize / 8;

        let sz = if height == 0 {
            0
        } else {
            stride * (height as usize - 1) + valid_stride
        };
        let buf = vec![0u8; sz];
        Some(Self {
            width,
            height,
            stride,
            buf,
            fmt: std::marker::PhantomData,
        })
    }

    pub fn from_owned(orig: impl OwnedImageStride<FMT>) -> Self {
        let width = orig.width();
        let height = orig.height();
        let stride = orig.stride();
        let buf: Vec<u8> = orig.into(); // move data
        Self::new(width, height, stride, buf).unwrap()
    }
}

/// Compile-time test to ensure ImageStruct implements Send.
fn _test_owned_image_implements_send<F: PixelFormat + Send>() {
    fn implements<T: Send>() {}
    implements::<OImage<F>>();
}

/// Compile-time test to ensure ImageStruct implements Stride.
fn _test_owned_image_implements_stride<F: PixelFormat>() {
    fn implements<T: Stride, F>() {}
    implements::<OImage<F>, F>();
}

/// Compile-time test to ensure ImageStruct implements Into<Vec<u8>>.
fn _test_owned_image_implements_into_vec_u8<F: PixelFormat>() {
    fn implements<T: Into<Vec<u8>>>() {}
    implements::<OImage<F>>();
}

impl<F: PixelFormat> std::fmt::Debug for OImage<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageStruct")
            .field("fmt", &self.fmt)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("stride", &self.stride)
            .finish_non_exhaustive()
    }
}

impl<F: PixelFormat> OImage<F> {
    pub fn copy_from<FRAME: crate::ImageStride<F>>(frame: &FRAME) -> OImage<F> {
        let width = frame.width();
        let height = frame.height();
        let stride = frame.stride();
        let buf = frame.image_data().to_vec(); // copy data

        Self {
            width,
            height,
            stride,
            buf,
            fmt: std::marker::PhantomData,
        }
    }
}

impl<F: PixelFormat> From<OImage<F>> for Vec<u8> {
    fn from(orig: OImage<F>) -> Vec<u8> {
        orig.buf
    }
}

impl<F: PixelFormat> From<Box<OImage<F>>> for Vec<u8> {
    fn from(orig: Box<OImage<F>>) -> Vec<u8> {
        orig.buf
    }
}

#[test]
fn test_alloc() {
    // test the key size of 0 and some other arbitrary size.
    for width in [0, 640] {
        for height in [0, 480] {
            let min_stride = (width * 3) as usize; // RGB8 has 3 bytes per pixel
            for stride in [min_stride, min_stride + 10] {
                // Test zeros.
                let img =
                    OImage::<crate::pixel_format::RGB8>::zeros(width, height, stride).unwrap();
                assert_eq!(img.width(), width);
                assert_eq!(img.height(), height);
                assert_eq!(img.stride(), stride);

                // Test new.
                let sz = height as usize * stride;
                let buf = vec![0u8; sz]; // allocate buffer
                let img =
                    OImage::<crate::pixel_format::RGB8>::new(width, height, stride, buf).unwrap();
                assert_eq!(img.width(), width);
                assert_eq!(img.height(), height);
                assert_eq!(img.stride(), stride);
            }
        }
    }
}
