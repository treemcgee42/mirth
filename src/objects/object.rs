
// S==== IMPORTS {{{1

use std::rc::Rc;
use crate::utility::{
    math::ray::Ray3, 
    rng::RandomNumberGenerator
};
use super::{
    shapes::traits::{ShapeLike, ShapeIntersectionInfo}, 
    textures::traits::TextureLike, 
    materials::traits::{MaterialLike, MaterialScatterResult}
};

// E==== IMPORTS }}}1

pub struct Object {
    pub(super) shape: Rc<dyn ShapeLike>,
    texture: Rc<dyn TextureLike>,
    material: Rc<dyn MaterialLike>,
}

/// Parameter to `Object::sample_new_ray()`.
pub struct SampleNewRayInfo<'a> {
    pub incoming_ray: &'a Ray3,
    pub shape_intersection: &'a ShapeIntersectionInfo,
    pub rng: &'a mut RandomNumberGenerator,
}

impl Object {
    pub fn sample_new_ray(&self, info: SampleNewRayInfo) -> MaterialScatterResult {
        self.material.scatter(info.incoming_ray, info.shape_intersection, info.rng)
    }
}

