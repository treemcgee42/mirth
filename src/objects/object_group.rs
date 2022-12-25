
// S==== IMPORTS {{{1

use std::rc::Rc;
use crate::utility::math::ray::Ray3;
use super::{
    object::Object,
    shapes::traits::ShapeIntersectionInfo
};

// E==== IMPORTS }}}1

pub struct ObjectGroup {
    objects: Vec<Rc<Object>>,
}

pub struct ObjectGroupIntersectionInfo {
    pub intersected_object: Option<Rc<Object>>,
    pub shape_intersection_info: ShapeIntersectionInfo,
}

impl ObjectGroup {
    pub fn new_from_vector(objects: Vec<Rc<Object>>) -> Self {
        Self { objects }
    }

    pub fn intersect(&self, ray: &Ray3) -> ObjectGroupIntersectionInfo {
        self.intersect_unoptimized(ray)
    }

    /// Go through each object in the scene and check for intersection.
    fn intersect_unoptimized(&self, ray: &Ray3) -> ObjectGroupIntersectionInfo {
        let mut working_ray = ray.clone();
        let mut to_return = ObjectGroupIntersectionInfo {
            intersected_object: None,
            shape_intersection_info: ShapeIntersectionInfo::default(),
        };

        for object in self.objects.iter() {
            let shape_intersection_info = object.shape.intersect(&working_ray);
            
            if !shape_intersection_info.did_hit { continue; }
            if shape_intersection_info.t > to_return.shape_intersection_info.t { continue; }

            working_ray.max_t = shape_intersection_info.t;
            to_return = ObjectGroupIntersectionInfo {
                intersected_object: Some(object.clone()),
                shape_intersection_info,
            };
        }

        to_return
    }
}

