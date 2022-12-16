use std::sync::Arc;
use crate::{light::Spectrum, utility::math::{ray::Ray3, vector::{Vec3, Color3}}};

use super::traits::{TextureLike, TextureCoordinates};

#[derive(Debug)]
pub struct ConstantTexture {
    color: Arc<Spectrum>, 
}

impl TextureLike for ConstantTexture {
    fn value_at(&self, _incoming_ray: &Ray3, _coordinate: &TextureCoordinates) -> Arc<Spectrum> {
        self.color.clone()
    }
}

impl ConstantTexture {
    pub fn new(color: Arc<Spectrum>) -> Self {
        Self {
            color
        }
    }

    pub fn new_from_rgb(rgb: Color3) -> Self {
        Self {
            color: Arc::new(rgb.clone()),
        }
    }
}

