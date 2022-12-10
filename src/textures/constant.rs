use std::sync::Arc;
use crate::{light::Spectrum, utility::math::ray::Ray3};

use super::traits::{TextureLike, TextureCoordinates};

pub struct ConstantTexture {
    color: Arc<Spectrum>, 
}

impl TextureLike for ConstantTexture {
    fn value_at(&self, _incoming_ray: &Ray3, _coordinate: &TextureCoordinates) -> Arc<Spectrum> {
        self.color.clone()
    }
}

