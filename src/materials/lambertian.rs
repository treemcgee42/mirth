use std::sync::Arc;

use crate::{textures::TextureLike, utility::linalg::{OrthonormalBasis, Ray3}, sampler, light::ConvertableToLight};

use super::{MaterialLike, ScatterResult};


pub struct Lambertian {
    texture: Arc<dyn TextureLike>
}

impl MaterialLike for Lambertian {
    fn scatter(
        &self,
        incoming_ray: &crate::utility::linalg::Ray3, 
        intersection_info: &crate::shapes::IntersectionInfo, 
        rng: &mut crate::utility::rng::RandomNumberGenerator
    ) -> super::ScatterResult {
        let sample_result = sampler::cosine_on_2sphere_hemisphere(rng);

        let scattered_direction = {
            let onb = OrthonormalBasis::new_from_vector(&intersection_info.surface_normal);
            onb.vector_from_local(sample_result.point.clone())
        };
        let scattered_ray = Ray3::new(intersection_info.point.clone(), scattered_direction);

        let light = self.texture.value_at(&incoming_ray, &intersection_info.texture_coordinates).to_light(0.0);

        ScatterResult {
            did_scatter: true,
            scattered_ray,
            pdf: sample_result.pdf,
            light,
        }
    }
}

