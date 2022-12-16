//! This encapsulates all the geometry of the scene. 


use crate::{camera::Camera, objects::object_group::ObjectGroup, integrators::traits::IntegratorLike, utility::{image::{Resolution, Image, ImageBuffer}, rng::RandomNumberGenerator, math::float::Float}};

pub struct Scene {
    resolution: Resolution,
    integrator: Box<dyn IntegratorLike>,
    camera: Camera,
    objects: ObjectGroup, 
    rng: RandomNumberGenerator,
    num_samples: usize,
}

impl Scene {
    fn new_from_json(json: &serde_json::Value) -> Result<Self, ()> {
        let camera: Camera;
        if let Ok(camera_) = Camera::new_from_json(&json["camera"]) {
            camera = camera_;
        } else {
            return Err(());
        }

        todo!()
    }

    fn ray_trace(&mut self) -> Image {
        let mut image_buffer = ImageBuffer::new(self.resolution.clone());

        while image_buffer.num_samples() < self.num_samples {
            image_buffer.add_sample(self.ray_trace_single_sample());
        }
        
        image_buffer.average_samples()
    }

    fn ray_trace_single_sample(&mut self) -> Image {
        let mut to_return = Image::new(self.resolution.clone());

        for pixel in self.resolution.clone().into_iter() {
            let camera_ray = {
                let px = (pixel.x as Float) + 0.5;
                let py = (pixel.y as Float) + 0.5;
                self.camera.generate_ray(px, py, &mut self.rng)
            };
            
            let pixel_color = self.integrator.spectrum_from_ray(&self.objects, &camera_ray, &mut self.rng);
            to_return.set_pixel_color(&pixel, pixel_color);
        }

        to_return
    }
}

#[cfg(test)]
mod tests {

    use crate::{camera::{self, Camera, CameraInfo}, objects::shapes::{transform::Transform, quad::Quad}, utility::{math::{vector::Vec3, angle::{AngleUnits, Angle}}, image::Resolution}};

    #[test]
    fn scene_1() {
        let camera_info = CameraInfo {
            transform: Transform::new_for_viewer(
                &Vec3::new(0.0,0.0,1.0), &Vec3::new(0.0,0.0,0.0), &Vec3::new(0.0,1.0,0.0)
            ),
            aperture_radius: 0.0,
            focal_distance: 1.0,
            resolution: Resolution { width: 600, height: 600 },
            vertical_fov: Angle { amount: 90.0, units: AngleUnits::Degrees }
        };
        let camera = Camera::new(camera_info);

    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{shapes::sphere::Sphere, utility::linalg::{Point3, Vec3}, config::SignCheckable};
//
//     use super::*;
//
//     #[test]
//     fn nearest_hit() {
//         // Ray: (0,0,0) + t(0,0,1)
//         //
//         // Sphere 1:    center: (0,0,2)     radius: 1
//         // Sphere 2:    center: (0,0,5)     radius: 1
//         // Sphere 3:    center: (0,0,8)     radius: 1
//         //
//         // Expected hit point: (0,0,1)
//
//     let ray = Ray3::new(Point3::origin(), Vec3::new(0.0,0.0,1.0));
//
//         let sphere_1 = Rc::new(Sphere::new(Point3::new(0.0,0.0,2.0), 1.0));
//         let sphere_2 = Rc::new(Sphere::new(Point3::new(0.0,0.0,5.0), 1.0));
//         let sphere_3 = Rc::new(Sphere::new(Point3::new(0.0,0.0,8.0), 1.0));
//
//         let scene = Scene {
//             objects: vec![sphere_2, sphere_1, sphere_3],
//         };
//
//         let hit_info = scene.intersect_unoptimized(&ray);
//
//         assert!(
//             hit_info.did_hit
//             && Point3::are_equal(&hit_info.point, &Point3::new(0.0,0.0,1.0))
//             && (hit_info.t - 1.0).is_zero()
//         );
//     }
// }

