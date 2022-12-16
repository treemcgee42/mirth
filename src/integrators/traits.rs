
use crate::{
    utility::{math::ray::Ray3, rng::RandomNumberGenerator}, 
    light::Spectrum, 
    objects::object_group::ObjectGroup
};


pub trait IntegratorLike {
    fn spectrum_from_ray(&self, object_group: &ObjectGroup, ray: &Ray3, rng: &mut RandomNumberGenerator) -> Spectrum;
}

