//! Collection of traits that define a shape. 

use crate::{
    objects::textures::traits::TextureCoordinates, 
    utility::math::{
        vector::{Point3, Vec3}, 
        float::Float, ray::Ray3}
};
use super::transform::Transform;

/// Rusty idiom for indicating that an implementor really should be keeping track of
/// a transform internally.
pub trait Transformable {
    fn get_transform(&self) -> Transform;
}

pub struct ShapeIntersectionInfo {
    pub did_hit: bool,
    pub point: Point3,
    pub t: Float,
    pub surface_normal: Vec3,
    pub texture_coordinates: TextureCoordinates,
}

impl Default for ShapeIntersectionInfo {
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

impl ShapeIntersectionInfo {
    pub fn no_intersection() -> Self {
        Self {
            did_hit: false,
            ..Default::default()
        }
    }
}

pub trait IntersectableShape {
    fn intersect(&self, ray: &Ray3) -> ShapeIntersectionInfo;
}

pub trait ShapeLike: IntersectableShape + Transformable {}

