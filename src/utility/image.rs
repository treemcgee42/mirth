//! Reading and writting to image formats

use image;
use serde::Deserialize;
use super::math::{vector::Color3, float::Float};

// S==== ASSOCIATED TYPES {{{1

#[derive(Clone, Debug, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl IntoIterator for Resolution {
    type Item = Pixel;
    type IntoIter = PixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterator {
            pixel: Pixel { x: 0, y: 0 },
            resolution: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
}

pub struct PixelIterator {
    pixel: Pixel,
    resolution: Resolution,
}

impl Iterator for PixelIterator {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pixel.x < self.resolution.width - 1 { 
            self.pixel = Pixel { x: self.pixel.x + 1, y: self.pixel.y };
            return Some(self.pixel.clone());
        }

        if self.pixel.y < self.resolution.height - 1 {
            self.pixel = Pixel { x: 0, y: self.pixel.y + 1 };
            return Some(self.pixel.clone());
        }

        return None;
    }
}

// E==== ASSOCIATED TYPES }}}1

// S==== IMAGE {{{1

/// Representation of an image, for decoding, writing, and encoding. Our convention is to 
/// begin counting on the bottom left, which we denote (0, 0), as is common in math. 
#[derive(Clone)]
pub struct Image {
    resolution: Resolution,
    internal: image::Rgb32FImage,
}

impl Image {
    /// Create a new, unitialized image. This will allocate enough memory to store
    /// the pixel buffer, but the pixels will not be initialized to a particular 
    /// value.
    pub fn new(resolution: Resolution) -> Image {
        let width = resolution.width;
        let height = resolution.height;

        Image {
            resolution,
            internal: image::Rgb32FImage::new(width, height),
        }
    }

    /// Set the pixel (x, y) to color. Recall that (0, 0) is in the bottom left.
    pub fn set_pixel_color(&mut self, pixel: &Pixel, color: Color3) {
        assert!(pixel.x < self.resolution.width && pixel.y < self.resolution.height, "out of bounds index");

        // convert to image crate convention, which has (0, 0) in the top left corner
        let xc = pixel.x;
        let yc = self.internal.height() - pixel.y - 1;

        self.internal.put_pixel(xc, yc, color.into());
    }

    pub fn get_pixel_color(&self, pixel: &Pixel) -> Color3 {
        assert!(pixel.x < self.resolution.width && pixel.y < self.resolution.height, "out of bounds index");

        // convert to image crate convention, which has (0, 0) in the top left corner
        let xc = pixel.x;
        let yc = self.internal.height() - pixel.y - 1;

        let pre_color = self.internal.get_pixel(xc, yc);
        Color3::new(pre_color.0[0], pre_color.0[1], pre_color.0[2])
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

// E==== IMAGE }}}1

// S==== IMAGE BUFFER {{{1

pub struct ImageBuffer {
    image: Image,
    samples: usize,
}

impl ImageBuffer {
    pub fn new(resolution: Resolution) -> Self {
        Self {
            image: Image::new(resolution),
            samples: 0
        }
    }

    pub fn num_samples(&self) -> usize {
        self.samples
    }

    pub fn add_sample(&mut self, image: Image) {
        for pixel in image.resolution.clone().into_iter() {
            self.image.set_pixel_color(
                &pixel,
                self.image.get_pixel_color(&pixel) + image.get_pixel_color(&pixel)
            );
        }
        
        self.samples += 1;
    }

    pub fn average_samples(&self) -> Image {
        let mut to_return = self.image.clone();

        for pixel in to_return.resolution.clone().into_iter() {
            to_return.set_pixel_color(
                &pixel,
                to_return.get_pixel_color(&pixel) / (self.samples as Float)
            );
        }

        to_return
    }

    /// Saves the image buffer to a file, whose encoding is deduced from the filename 
    /// (so include the extension in `filename`).
    pub fn save_to_file(&self, filename: &str) -> Result<(), String> {
        let image = self.average_samples();
        image.save_to_file(filename)
    }
}

// E==== IMAGE BUFFER }}}1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_iterator() {
        let resolution = Resolution { width: 5, height: 5 };
        for pixel in resolution.into_iter() {
            print!("({},{}) ", pixel.x, pixel.y);
        }
        print!("\n");
    }
}

