//! Reading and writting to image formats

use image;
use crate::utility::linalg;

/// Representation of an image, for decoding, writing, and encoding. Our convention is to 
/// begin counting on the bottom left, which we denote (0, 0), as is common in math. 
pub struct Image {
    width: u32,
    height: u32,
    internal: image::Rgb32FImage,
}

impl Image {
    /// Create a new, unitialized image. This will allocate enough memory to store
    /// the pixel buffer, but the pixels will not be initialized to a particular 
    /// value.
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            internal: image::Rgb32FImage::new(width, height),
        }
    }

    /// Set the pixel (x, y) to color. Recall that (0, 0) is in the bottom left.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: linalg::Color3) {
        assert!(x < self.width && y < self.height, "out of bounds index");

        // convert to image crate convention, which has (0, 0) in the top left corner
        let xc = x;
        let yc = self.internal.height() - y - 1;

        self.internal.put_pixel(xc, yc, color.into());
    }

    /// Saves the image buffer to a file, whose encoding is deduced from the filename 
    /// (so include the extension in `filename`).
    pub fn save_to_file(&self, filename: &str) -> Result<(), String> {
        match self.internal.save(filename) {
            Ok(_) => { return Ok(()); }
            Err(e) => { return Err(e.to_string()); }
        }
    }
}

