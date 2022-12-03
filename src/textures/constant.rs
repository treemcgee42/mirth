use std::sync::Arc;

use crate::{light::Spectrum, utility::linalg::Ray3};

use super::TextureLike;


pub struct ConstantTexture {
    color: Arc<Spectrum>, 
}

impl TextureLike for ConstantTexture {
    fn value_at(&self, _incoming_ray: &Ray3, _coordinate: &super::TextureCoordinates) -> Arc<Spectrum> {
        self.color.clone()
    }
}

