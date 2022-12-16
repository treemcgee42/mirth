use crate::{
    utility::{
        math::{float::Float, ray::Ray3}, 
        rng::RandomNumberGenerator
    }, 
    objects::shapes::traits::ShapeIntersectionInfo
};

pub struct MaterialScatterResult {
    pub did_scatter: bool,
    pub scattered_ray: Ray3,
    pub pdf: Float,
}

pub trait MaterialLike {
    fn scatter(
        &self,
        incoming_ray: &Ray3, 
        shape_intersection_info: &ShapeIntersectionInfo, 
        rng: &mut RandomNumberGenerator
    ) -> MaterialScatterResult;
}
