//! Copy-on-Write (CoW) image that can either borrow or own its pixel data.

use crate::{
    image_ref::ImageRef, owned::OImage, ImageBuffer, ImageBufferRef, ImageData, PixelFormat, Stride,
};

/// A Copy-on-Write (CoW) image that can either borrow or own its pixel data.
///
/// `CowImage` provides a flexible way to work with image data that may be either
/// borrowed from an existing source or owned by the container. This is particularly
/// useful in scenarios where you want to avoid unnecessary copying when possible,
/// but still provide owned data when needed.
///
/// The enum has two variants:
/// - `Borrowed`: Contains an [`ImageRef`] that borrows data from elsewhere
/// - `Owned`: Contains an [`OImage`] that owns its pixel data
///
/// Both variants implement the same image traits, allowing them to be used
/// interchangeably in most contexts.
///
/// # Type Parameters
/// * `F` - The pixel format type that implements [`PixelFormat`]
///
/// # Examples
/// ```rust
/// # use machine_vision_formats::{cow::CowImage, owned::OImage, image_ref::ImageRef, pixel_format::Mono8};
/// // Create from borrowed data
/// let data = [128u8; 100];
/// let borrowed_ref = ImageRef::<Mono8>::new(10, 10, 10, &data).unwrap();
/// let cow_borrowed = CowImage::from(borrowed_ref);
///
/// // Create from owned data
/// let owned_image = OImage::<Mono8>::new(10, 10, 10, vec![64u8; 100]).unwrap();
/// let cow_owned = CowImage::from(owned_image);
/// ```
pub enum CowImage<'a, F: PixelFormat> {
    /// Borrowed image data with a lifetime tied to the source
    Borrowed(ImageRef<'a, F>),
    /// Owned image data that manages its own memory
    Owned(OImage<F>),
}

impl<'a, F: PixelFormat> CowImage<'a, F> {
    /// Consumes the image and returns an owned image.
    ///
    /// For borrowed images, this copies the data into a new image.
    /// For owned images, this moves the existing data without copying.
    ///
    /// # Examples
    /// ```rust
    /// # use machine_vision_formats::{cow::CowImage, owned::OImage, pixel_format::Mono8, ImageData};
    /// let owned_image = OImage::<Mono8>::new(4, 4, 4, vec![255u8; 16]).unwrap();
    /// let cow_image = CowImage::from(owned_image);
    /// let owned = cow_image.owned();
    /// ```
    pub fn owned(self) -> OImage<F> {
        match self {
            CowImage::Borrowed(im) => {
                let w = im.width();
                let h = im.height();
                let s = im.stride();
                let buf = im.buffer();
                crate::owned::OImage::new(w, h, s, buf.data).unwrap()
            }
            CowImage::Owned(im) => im,
        }
    }
}

impl<'a, F: PixelFormat> From<ImageRef<'a, F>> for CowImage<'a, F> {
    /// Creates a [`CowImage::Borrowed`] from an [`ImageRef`].
    ///
    /// # Examples
    /// ```rust
    /// # use machine_vision_formats::{cow::CowImage, image_ref::ImageRef, pixel_format::RGB8};
    /// let data = [0u8; 300]; // 10x10 RGB image
    /// let image_ref = ImageRef::<RGB8>::new(10, 10, 30, &data).unwrap();
    /// let cow_image = CowImage::from(image_ref);
    /// ```
    fn from(frame: ImageRef<'a, F>) -> CowImage<'a, F> {
        CowImage::Borrowed(frame)
    }
}

impl<'a, F: PixelFormat> From<OImage<F>> for CowImage<'a, F> {
    /// Creates a [`CowImage::Owned`] from an [`OImage`].
    ///
    /// # Examples
    /// ```rust
    /// # use machine_vision_formats::{cow::CowImage, owned::OImage, pixel_format::Mono8};
    /// let owned_image = OImage::<Mono8>::new(20, 15, 20, vec![128u8; 300]).unwrap();
    /// let cow_image = CowImage::from(owned_image);
    /// ```
    fn from(frame: OImage<F>) -> CowImage<'a, F> {
        CowImage::Owned(frame)
    }
}

impl<F: PixelFormat> Stride for CowImage<'_, F> {
    /// Returns the stride (bytes per row) of the image.
    ///
    /// # Examples
    /// ```rust
    /// # use machine_vision_formats::{cow::CowImage, owned::OImage, pixel_format::Mono8, Stride};
    /// let owned_image = OImage::<Mono8>::new(10, 10, 12, vec![0u8; 120]).unwrap();
    /// let cow_image = CowImage::from(owned_image);
    /// assert_eq!(cow_image.stride(), 12);
    /// ```
    fn stride(&self) -> usize {
        match self {
            CowImage::Borrowed(im) => im.stride(),
            CowImage::Owned(im) => im.stride(),
        }
    }
}

impl<F: PixelFormat> ImageData<F> for CowImage<'_, F> {
    /// Returns the width of the image in pixels.
    ///
    /// # Examples
    /// ```rust
    /// # use machine_vision_formats::{cow::CowImage, owned::OImage, pixel_format::RGB8, ImageData};
    /// let owned_image = OImage::<RGB8>::new(25, 20, 75, vec![0u8; 1500]).unwrap();
    /// let cow_image = CowImage::from(owned_image);
    /// assert_eq!(cow_image.width(), 25);
    /// ```
    fn width(&self) -> u32 {
        match self {
            CowImage::Borrowed(im) => im.width(),
            CowImage::Owned(im) => im.width(),
        }
    }

    /// Returns the height of the image in pixels.
    ///
    /// # Examples
    /// ```rust
    /// # use machine_vision_formats::{cow::CowImage, image_ref::ImageRef, pixel_format::Mono8, ImageData};
    /// let data = [0u8; 200];
    /// let image_ref = ImageRef::<Mono8>::new(10, 20, 10, &data).unwrap();
    /// let cow_image = CowImage::from(image_ref);
    /// assert_eq!(cow_image.height(), 20);
    /// ```
    fn height(&self) -> u32 {
        match self {
            CowImage::Borrowed(im) => im.height(),
            CowImage::Owned(im) => im.height(),
        }
    }

    /// Returns a reference to the image buffer.
    ///
    /// This provides access to the pixel data regardless of whether the image
    /// is borrowed or owned.
    ///
    /// # Examples
    /// ```rust
    /// # use machine_vision_formats::{cow::CowImage, owned::OImage, pixel_format::Mono8, ImageData};
    /// let owned_image = OImage::<Mono8>::new(5, 5, 5, vec![42u8; 25]).unwrap();
    /// let cow_image = CowImage::from(owned_image);
    /// let buffer_ref = cow_image.buffer_ref();
    /// ```
    fn buffer_ref(&self) -> ImageBufferRef<'_, F> {
        let image_data = match self {
            CowImage::Borrowed(im) => im.image_data(),
            CowImage::Owned(im) => im.image_data(),
        };
        ImageBufferRef::new(image_data)
    }

    /// Consumes the image and returns the pixel data as an owned buffer.
    ///
    /// For borrowed images, this copies the data into a new vector.
    /// For owned images, this moves the existing data without copying.
    ///
    /// # Examples
    /// ```rust
    /// # use machine_vision_formats::{cow::CowImage, owned::OImage, pixel_format::Mono8, ImageData};
    /// let owned_image = OImage::<Mono8>::new(4, 4, 4, vec![255u8; 16]).unwrap();
    /// let cow_image = CowImage::from(owned_image);
    /// let buffer = cow_image.buffer();
    /// ```
    fn buffer(self) -> ImageBuffer<F> {
        match self {
            CowImage::Borrowed(im) => ImageBuffer::new(im.image_data().to_vec()),
            CowImage::Owned(im) => ImageBuffer::new(im.into()),
        }
    }
}
