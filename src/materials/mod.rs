//! Each shape should have a material. When a ray intersects a surface, the 
//! material determines how that ray scatters.

pub mod lambertian;

use crate::{utility::{linalg::Ray3, rng::RandomNumberGenerator}, shapes::IntersectionInfo, config::Float, light::Light};

pub struct ScatterResult {
    pub did_scatter: bool,
    pub scattered_ray: Ray3,
    pub pdf: Float,
    pub light: Light,
}

pub trait MaterialLike {
    fn scatter(
        &self,
        incoming_ray: &Ray3, 
        intersection_info: &IntersectionInfo, 
        rng: &mut RandomNumberGenerator
    ) -> ScatterResult;
}

