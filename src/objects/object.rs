
// S==== IMPORTS {{{1

use std::{rc::Rc, collections::HashMap};
use crate::utility::{
    math::ray::Ray3, 
    rng::RandomNumberGenerator
};
use super::{
    shapes::{traits::{ShapeLike, ShapeIntersectionInfo}, quad::Quad, self}, 
    textures::traits::TextureLike, 
    materials::traits::{MaterialLike, MaterialScatterResult}
};
use tracing::error;

// E==== IMPORTS }}}1

pub struct Object {
    pub(super) shape: Rc<dyn ShapeLike>,
    texture: Rc<dyn TextureLike>,
    material: Rc<dyn MaterialLike>,
}

pub struct ObjectInfo {
    pub shape: Rc<dyn ShapeLike>,
    pub texture: Rc<dyn TextureLike>,
    pub material: Rc<dyn MaterialLike>,
}

/// Parameter to `Object::sample_new_ray()`.
pub struct SampleNewRayInfo<'a> {
    pub incoming_ray: &'a Ray3,
    pub shape_intersection: &'a ShapeIntersectionInfo,
    pub rng: &'a mut RandomNumberGenerator,
}

impl Object {
    pub fn new(info: ObjectInfo) -> Self {
        Self {
            shape: info.shape,
            texture: info.texture,
            material: info.material
        }
    }

    pub fn sample_new_ray(&self, info: SampleNewRayInfo) -> MaterialScatterResult {
        self.material.scatter(info.incoming_ray, info.shape_intersection, info.rng)
    }
}

