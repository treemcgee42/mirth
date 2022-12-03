//! This encapsulates all the geometry of the scene. 

use crate::{shapes::{SurfaceLike, IntersectionInfo}, utility::linalg::Ray3};
use std::rc::Rc;

pub struct Scene {
    objects: Vec<Rc<dyn SurfaceLike>>, 
}

impl Scene {
    /// Go through each object in the scene and check for intersection.
    fn intersect_unoptimized(&self, ray: &Ray3) -> IntersectionInfo {
        let mut working_ray = ray.clone();
        let mut to_return = IntersectionInfo::default(); 

        for object in self.objects.iter() {
            let object_hit_info = object.intersect(&working_ray);
            
            if !object_hit_info.did_hit { continue; }
            if object_hit_info.t > to_return.t { continue; }

            working_ray.max_t = object_hit_info.t;
            to_return = object_hit_info;
        }

        to_return
    }
}

#[cfg(test)]
mod tests {
    use crate::{shapes::sphere::Sphere, utility::linalg::{Point3, Vec3}, config::SignCheckable};

    use super::*;

    #[test]
    fn nearest_hit() {
        // Ray: (0,0,0) + t(0,0,1)
        //
        // Sphere 1:    center: (0,0,2)     radius: 1
        // Sphere 2:    center: (0,0,5)     radius: 1
        // Sphere 3:    center: (0,0,8)     radius: 1
        //
        // Expected hit point: (0,0,1)

        let ray = Ray3::new(Point3::origin(), Vec3::new(0.0,0.0,1.0));

        let sphere_1 = Rc::new(Sphere::new(Point3::new(0.0,0.0,2.0), 1.0));
        let sphere_2 = Rc::new(Sphere::new(Point3::new(0.0,0.0,5.0), 1.0));
        let sphere_3 = Rc::new(Sphere::new(Point3::new(0.0,0.0,8.0), 1.0));

        let scene = Scene {
            objects: vec![sphere_2, sphere_1, sphere_3],
        };

        let hit_info = scene.intersect_unoptimized(&ray);

        assert!(
            hit_info.did_hit
            && Point3::are_equal(&hit_info.point, &Point3::new(0.0,0.0,1.0))
            && (hit_info.t - 1.0).is_zero()
        );
    }
}

