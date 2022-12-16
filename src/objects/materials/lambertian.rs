
// S==== IMPORTS {{{1

use crate::{
    objects::shapes::traits::ShapeIntersectionInfo,
    utility::math::{
        ray::Ray3, 
        orthonormal_basis::OrthonormalBasis
    }, 
    sampler, 
};
use super::traits::{MaterialLike, MaterialScatterResult};

// E==== IMPORTS }}}1

pub struct Lambertian {
}

impl MaterialLike for Lambertian {
    fn scatter(
        &self,
        incoming_ray: &Ray3, 
        shape_intersection_info: &ShapeIntersectionInfo, 
        rng: &mut crate::utility::rng::RandomNumberGenerator
    ) -> MaterialScatterResult {
        let sample_result = sampler::cosine_on_2sphere_hemisphere(rng);

        let scattered_direction = {
            let onb = OrthonormalBasis::new_from_vector(&shape_intersection_info.surface_normal);
            onb.vector_from_local(sample_result.point.clone())
        };
        let scattered_ray = Ray3::new(shape_intersection_info.point.clone(), scattered_direction);


        MaterialScatterResult {
            did_scatter: true,
            scattered_ray,
            pdf: sample_result.pdf,
        }
    }
}

