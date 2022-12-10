//! This encapsulates all the geometry of the scene. 

use crate::{camera::Camera, shapes::shapes::Shapes};

pub struct Scene {
    camera: Camera,
    surfaces: Shapes, 
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

