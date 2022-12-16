use std::{sync::Arc, fmt::Debug};
use crate::{
    utility::math::{float::Float, ray::Ray3, vector::Vec3}, 
    light::Spectrum
};

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

pub trait TextureLike: Debug {
    fn value_at(&self, incoming_ray: &Ray3, coordinate: &TextureCoordinates) -> Arc<Spectrum>;
}

