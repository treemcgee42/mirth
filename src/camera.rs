//! Mirth treats the camera as an object in its own right. Local to camera 
//! space, the image is centered at $(0,0,-1)$. The unit conversion from pixels
//! is 1-1, so the top left corner is at $(-w/2,h/2,-1)$ and the bottom 
//! right corner is at $(w/2,-h/2,-1)$. 

use serde::Deserialize;
use crate::{
    shapes::transform::Transform, 
    utility::{
        math::{
            vector::Vec3, 
            ray::Ray3,
            float::Float, 
            angle::{Angle, AngleUnits}
        }, 
        rng::RandomNumberGenerator, 
    },
    sampler
};

pub struct Camera {
    resolution: Resolution,
    transform: Transform,
    viewport_size: ViewportSize,
    bottom_left_corner_of_image_plane: Vec3,
    focal_distance: Float,
    aperture_radius: Float,
}

#[derive(Deserialize)]
pub struct Resolution {
    width: Float,
    height: Float,
}

pub struct ViewportSize {
    width: Float,
    height: Float,
}

// S==== PARSING {{{1

impl Camera {
    /// ```
    /// "camera": {
    ///     "resolution": [Float, Float],
    ///     "focal distance": Float,
    ///     "vertical fov": Float,
    ///     "aperture radius": Float,
    ///     "transform": ViewerTransform
    /// }
    /// ```
    pub fn new_from_json(json: &serde_json::Value) -> Result<Self, ()> {
        let resolution: Resolution;
        if let Ok(resolution_) = serde_json::from_value::<Resolution>(json["resolution"].clone()) {
            resolution = resolution_;
        } else { 
            return Err(()); 
        }

        let focal_distance: Float;
        if let Ok(focal_distance_) = serde_json::from_value::<Float>(json["focal distance"].clone()) {
            focal_distance = focal_distance_;
        } else {
            return Err(());
        }

        let vertical_fov: Angle;
        if let Ok(vertical_fov_) = serde_json::from_value::<Float>(json["vertical fov"].clone()) {
            vertical_fov = Angle {
                amount: vertical_fov_,
                units: AngleUnits::Degrees,
            };
        } else {
            return Err(());
        }

        let aperture_radius: Float;
        if let Ok(aperture_radius_) = serde_json::from_value::<Float>(json["aperture radius"].clone()) {
            aperture_radius = aperture_radius_;
        } else {
            return Err(());
        }

        let transform: Transform;
        if let Ok(transform_) = Transform::new_from_json(&json["transform"]) {
            transform = transform_;
        } else { 
            return Err(()); 
        }

        let info = CameraInfo {
            transform,
            resolution,
            focal_distance,
            aperture_radius,
            vertical_fov,
        };

        Ok(Camera::new(info))
    }
}

// E==== PARSING }}}1

pub struct CameraInfo {
    pub transform: Transform,
    pub resolution: Resolution,
    /// The distance from the camera to the focal plane. The focal plane is the plane (to which 
    /// a straight ray from the camera is orthogonal) in which everything is in focus. The farther
    /// something is from the focal plane, the more blurry it will appear.
    pub focal_distance: Float,
    pub aperture_radius: Float,
    /// The angle between the ray from the viewer to the highest visible point and the ray from 
    /// the viewer to the lowest visible point (if you're imagining the viewer as a person, 
    /// we don't allow the person to move their head or eyes).
    pub vertical_fov: Angle,
}

impl Camera {
    pub fn new(info: CameraInfo) -> Self {
        let transform = info.transform;
        let resolution = info.resolution;
        let viewport_size = {
            let aspect_ratio = resolution.width / resolution.height;

            let height = {
                let theta = info.vertical_fov.as_radians();
                let height = Float::tan(theta / 2.0);
                2.0 * height
            };
            let width = aspect_ratio * height;

            ViewportSize { width, height }
        };
        let bottom_left_corner_of_image_plane = Vec3::new(
            0.5 * (-viewport_size.width),
            0.5 * (-viewport_size.height),
            -1.0
        );
        let focal_distance = info.focal_distance;
        let aperture_radius = info.aperture_radius;

        Self {
            transform,
            resolution,
            viewport_size,
            bottom_left_corner_of_image_plane,
            focal_distance,
            aperture_radius,
        }
    }
}

impl Camera {
    /// Returns a ray, in world space, from the camera that represents a 
    /// contribution to the pixel (pixel_x,pixel_y). Following the Mirth 
    /// convention, the bottom left pixel is (0,0), and the top right pixel is 
    /// (width-1,height-1).
    pub fn generate_ray(&self, pixel_x: Float, pixel_y: Float, rng: &mut RandomNumberGenerator) -> Ray3 {
        // --- Defocus blur ---
        // To achieve this effect, we offset the origin of the ray to represent
        // light passing through the lens from, potentially, somewhere other 
        // than the center. The direction of the ray is determined by the point 
        // on the focus plane that a ray centered at the origin (no offset) would
        // have intersected the focus plane at. This has the effect that the 
        // intersection of these offset rays with things on the focus plane is the 
        // same intersection you would have gotten with a non-offset ray. The farther
        // you get from the focus plane, the larger the difference between the offset
        // ray intersection and a non-offset ray intersection.

        // These are numbers between 0 and 1.
        let tx = pixel_x / self.resolution.width;
        let ty = pixel_y / self.resolution.height;

        let local_ray_origin = self.aperture_radius * sampler::uniform_in_1sphere(rng).point;
        let local_ray_direction = {
            let pixel_in_image_plane = 
                &self.bottom_left_corner_of_image_plane 
                + Vec3::new(tx * self.viewport_size.width, ty * self.viewport_size.height, 0.0);

            // Intersection of ray from camera to `pixel_in_image_plane` with the focus plane. 
            // Since we are in local space, and the image plane is z=-1, the focus plane is 
            // z=-(focal_distance) and so we can use the following shortcut to calculate the 
            // intersection with the focus plane.
            let focus_plane_intersection = self.focal_distance * pixel_in_image_plane;

            focus_plane_intersection - &local_ray_origin
        };
        let local_ray = Ray3::new(local_ray_origin, local_ray_direction);

        self.transform.ray_to_global(&local_ray)
    }
}
