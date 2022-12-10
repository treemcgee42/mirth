use crate::{
    utility::{
        math::{float::Float, ray::Ray3}, 
        rng::RandomNumberGenerator
    }, 
    shapes::traits::IntersectionInfo, 
    light::Light
};

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
