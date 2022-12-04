//! Mirth treats the camera as an object in its own right. Local to camera 
//! space, the image is centered at $(0,0,-1)$. The unit conversion from pixels
//! is 1-1, so the top left corner is at $(-w/2,h/2,-1)$ and the bottom 
//! right corner is at $(w/2,-h/2,-1)$. 

use crate::{shapes::transform::Transform, utility::linalg::{Ray3, Vec3, Point3}, config::Float};

pub struct Camera {
    picture_width: usize,
    picture_height: usize,
    top_left_corner_of_picture: Vec3,
    transform: Transform,
}

impl Camera {
    pub fn new_from_json(json: &serde_json::Value) {
       todo!() 
    }
}

impl Camera {
    /// Returns a ray, in world space, from the camera that represents a 
    /// contribution to the pixel (pixel_x,pixel_y). Following the Mirth 
    /// convention, the bottom left pixel is (0,0), and the top right pixel is 
    /// (width-1,height-1).
    pub fn generate_ray(&self, pixel_x: usize, pixel_y: usize) -> Ray3 {
        let point_in_pixel = {
            let pixel_as_vector = Vec3::new(
                pixel_x as Float,
                pixel_y as Float,
                -1.0
            );

            let bottom_left_corner_of_pixel = &self.top_left_corner_of_picture + pixel_as_vector;
            // For now, we will adjust to the center of this square.
            let adjustment = Vec3::new(0.5, 0.5, 0.0);

            bottom_left_corner_of_pixel + adjustment
        };

        let ray_in_camera_space = Ray3::new(Point3::origin(), point_in_pixel);

        self.transform.ray_to_global(&ray_in_camera_space)
    }
}
