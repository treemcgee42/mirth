
// S==== IMPORTS {{{1

use crate::{
    utility::{math::ray::Ray3, rng::RandomNumberGenerator}, 
    light::{Spectrum, ColorConstantsQueryable}, 
    objects::{object_group::ObjectGroup, object::SampleNewRayInfo}
};
use super::traits::IntegratorLike;

// E==== IMPORTS }}}1

struct AmbientOcclusionIntegrator {}

impl IntegratorLike for AmbientOcclusionIntegrator {
    fn spectrum_from_ray(&self, object_group: &ObjectGroup, ray: &Ray3, rng: &mut RandomNumberGenerator) -> Spectrum {
        /* Check if ray intersects any objects */
        
        let intersection_info = object_group.intersect(ray);
        if let None = intersection_info.intersected_object {
            return Spectrum::black();
        }

        let intersected_object = intersection_info.intersected_object.unwrap();

        /* Sample one more ray to see if the intersected point lies in shadow */

        let sample_result = {
            let info = SampleNewRayInfo {
                incoming_ray: ray,
                shape_intersection: &intersection_info.shape_intersection_info,
                rng,
            };

            intersected_object.sample_new_ray(info)
        };

        let shadow_ray = sample_result.scattered_ray;
        let shadow_intersection = object_group.intersect(&shadow_ray);

        if let Some(_) = shadow_intersection.intersected_object {
            return Spectrum::black();
        } else {
            return Spectrum::white();
        }
    }
}

