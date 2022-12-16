use std::rc::Rc;

use crate::utility::math::{vector::{Point3, Vec3}, float::Float, ray::Ray3};

use super::{textures::traits::TextureCoordinates, object::Object};


pub struct ObjectIntersectionInfo {
    /// If there was no intersection, this value is `None`.
    pub intersected_object: Option<Rc<Object>>,
    pub point: Point3,
    pub t: Float,
    pub surface_normal: Vec3,
    pub texture_coordinates: TextureCoordinates,
}

impl Default for ObjectIntersectionInfo {
    fn default() -> Self {
        Self {
            intersected_object: None,
            point: Point3::default(),
            t: Float::INFINITY,
            surface_normal: Vec3::new(0.0,0.0,0.0),
            texture_coordinates: TextureCoordinates::default(),
        }
    }
}

impl ObjectIntersectionInfo {
    pub fn no_intersection() -> Self {
        Self {
            intersected_object: None,
            ..Default::default()
        }
    }
}

pub trait IntersectableObject {
    fn intersect(&self, ray: &Ray3) -> ObjectIntersectionInfo;
}

