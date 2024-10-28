//! References to image data

use crate::{ImageBufferMutRef, ImageBufferRef, ImageData, ImageMutData, PixelFormat, Stride};

// -----

/// A view of image to have pixel format `FMT`.
pub struct ImageRef<'a, FMT: PixelFormat> {
    buf: &'a [u8],
    width: u32,
    height: u32,
    stride: usize,
    fmt: std::marker::PhantomData<FMT>,
}

impl<'a, FMT: PixelFormat> ImageRef<'a, FMT> {
    /// Use a `&[u8]` slice as the backing store for an ImageRef.
    ///
    /// Returns None if the buffer is not large enough to store an image of the
    /// desired properties.
    pub fn new(width: u32, height: u32, stride: usize, buf: &'a [u8]) -> Option<Self> {
        let fmt = crate::pixel_format::pixfmt::<FMT>().unwrap();
        let min_stride = fmt.bits_per_pixel() as usize * width as usize / 8;

        if height == 0 {
            return None;
        }
        let sz = stride * (height as usize - 1) + min_stride;

        if buf.len() < sz {
            return None;
        }
        Some(Self {
            width,
            height,
            stride,
            buf,
            fmt: std::marker::PhantomData,
        })
    }
}

impl<F: PixelFormat> std::fmt::Debug for ImageRef<'_, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageRef")
            .field("fmt", &self.fmt)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("stride", &self.stride)
            .finish_non_exhaustive()
    }
}

impl<'a, FMT: PixelFormat> ImageData<FMT> for ImageRef<'a, FMT> {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn buffer_ref(&self) -> ImageBufferRef<'_, FMT> {
        ImageBufferRef::new(self.buf)
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn buffer(self) -> crate::ImageBuffer<FMT> {
        // copy the data
        self.buffer_ref().to_buffer()
    }
}

impl<'a, FMT: PixelFormat> Stride for ImageRef<'a, FMT> {
    fn stride(&self) -> usize {
        self.stride
    }
}

// -----

/// A view of mutable image to have pixel format `FMT`.
pub struct ImageRefMut<'a, FMT: PixelFormat> {
    buf: &'a mut [u8],
    width: u32,
    height: u32,
    stride: usize,
    fmt: std::marker::PhantomData<FMT>,
}

impl<'a, FMT: PixelFormat> ImageRefMut<'a, FMT> {
    /// Use a `&mut [u8]` slice as the backing store for an ImageRefMut.
    ///
    /// Returns None if the buffer is not large enough to store an image of the
    /// desired properties.
    pub fn new(width: u32, height: u32, stride: usize, buf: &'a mut [u8]) -> Option<Self> {
        let fmt = crate::pixel_format::pixfmt::<FMT>().unwrap();
        let min_stride = fmt.bits_per_pixel() as usize * width as usize / 8;

        if height == 0 {
            return None;
        }
        let sz = stride * (height as usize - 1) + min_stride;

        if buf.len() < sz {
            return None;
        }
        Some(Self {
            width,
            height,
            stride,
            buf,
            fmt: std::marker::PhantomData,
        })
    }
}

impl<F: PixelFormat> std::fmt::Debug for ImageRefMut<'_, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageRefMut")
            .field("fmt", &self.fmt)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("stride", &self.stride)
            .finish_non_exhaustive()
    }
}

impl<'a, FMT: PixelFormat> ImageData<FMT> for ImageRefMut<'a, FMT> {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn buffer_ref(&self) -> ImageBufferRef<'_, FMT> {
        ImageBufferRef::new(self.buf)
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn buffer(self) -> crate::ImageBuffer<FMT> {
        // copy the data
        self.buffer_ref().to_buffer()
    }
}

impl<'a, FMT: PixelFormat> ImageMutData<FMT> for ImageRefMut<'a, FMT> {
    fn buffer_mut_ref(&mut self) -> ImageBufferMutRef<'_, FMT> {
        ImageBufferMutRef::new(self.buf)
    }
}

impl<'a, FMT: PixelFormat> Stride for ImageRefMut<'a, FMT> {
    fn stride(&self) -> usize {
        self.stride
    }
}
