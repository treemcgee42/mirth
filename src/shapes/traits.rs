//! Collection of traits that define a shape. 

use crate::{
    utility::math::{
        float::Float, 
        vector::{Point3, Vec3}, 
        ray::Ray3
    }, 
    textures::traits::TextureCoordinates
};
use super::transform::Transform;

#[derive(Debug)]
pub struct IntersectionInfo {
    pub did_hit: bool,
    pub point: Point3,
    pub t: Float,
    pub surface_normal: Vec3,
    pub texture_coordinates: TextureCoordinates,
}

impl Default for IntersectionInfo {
    fn default() -> Self {
        Self {
            did_hit: false,
            point: Point3::default(),
            t: Float::INFINITY,
            surface_normal: Vec3::new(0.0,0.0,0.0),
            texture_coordinates: TextureCoordinates::default(),
        }
    }
}

impl IntersectionInfo {
    pub fn no_intersection() -> Self {
        Self {
            did_hit: false,
            ..Default::default()
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray3) -> IntersectionInfo;
}

/// Rusty idiom for indicating that an implementor really should be keeping track of
/// a transform internally.
pub trait Transformable {
    fn get_transform(&self) -> Transform;
}

pub trait ShapeLike: Intersectable + Transformable {}

