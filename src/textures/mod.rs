use std::sync::Arc;

use crate::{light::{Light, Spectrum}, utility::linalg::{Ray3, Vec3}, config::Float};


pub mod constant;

#[derive(Debug)]
pub struct TextureCoordinates {
    u: Float,
    v: Float,
    normal: Vec3,
}

impl TextureCoordinates {
    pub fn default() -> Self {
        TextureCoordinates{
            u: 0.0,
            v: 0.0,
            normal: Vec3::new(0.0,0.0,0.0)
        }
    }
}

pub trait TextureLike {
    fn value_at(&self, incoming_ray: &Ray3, coordinate: &TextureCoordinates) -> Arc<Spectrum>;
}

