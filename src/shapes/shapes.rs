use std::rc::Rc;
use crate::utility::math::ray::Ray3;
use super::traits::{ShapeLike, IntersectionInfo, Intersectable};

/// Stores all the shapes in the scene. This hides the particular acceleration 
/// structure used, and determines that on its own when intersected.
pub struct Shapes {
    surfaces: Vec<Rc<dyn ShapeLike>>,
}

impl Shapes {
    /// "surfaces" = [
    ///     {
    ///         "type": SurfaceType,
    ///         "material": Material,
    ///         "transform": Transform
    ///     }
    /// ]
    pub fn new_from_json(json: &serde_json::Value) -> Result<Self, ()> {
        todo!()
    }
}

impl Shapes {
    /// Go through each object in the scene and check for intersection.
    fn intersect_unoptimized(&self, ray: &Ray3) -> IntersectionInfo {
        let mut working_ray = ray.clone();
        let mut to_return = IntersectionInfo::default(); 

        for object in self.surfaces.iter() {
            let object_hit_info = object.intersect(&working_ray);
            
            if !object_hit_info.did_hit { continue; }
            if object_hit_info.t > to_return.t { continue; }

            working_ray.max_t = object_hit_info.t;
            to_return = object_hit_info;
        }

        to_return
    }
}

impl Intersectable for Shapes {
    fn intersect(&self, ray: &Ray3) -> IntersectionInfo {
        self.intersect_unoptimized(ray)
    }
}
