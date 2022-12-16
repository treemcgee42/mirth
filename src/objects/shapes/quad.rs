
// S==== IMPORTS {{{1

use crate::{
    utility::math::{
        float::{Float, SignCheckable},
        vector::Vec3,
        ray::Ray3
    }, objects::textures::traits::TextureCoordinates, 
     
};
use super::{
    transform::Transform, 
    traits::{Transformable, ShapeLike, IntersectableShape, ShapeIntersectionInfo}
};

// E==== IMPORTS }}}1

pub struct Quad {
    pub width: Float,
    pub height: Float,
    pub transform: Transform,
}

impl IntersectableShape for Quad {
    /// In local space, the quad is defined by the corners $(0,0,0)$ and 
    /// $(w,h,0)$. Let $r=o+td$ be the ray we are intersecting against, in local 
    /// space. We do not consider the origin of $r$ lying within the quad as an 
    /// intersection. So the only way $r$ could intersect the quad is if its 
    /// $z$-component was variable, i.e. the $z$-component of $d$ is nonzero. 
    /// We can then get the time of intersection of $r$ with the plane $z=0$ by dividing
    /// the distance from $o$ and $z=0$ (which is the absolute value of the 
    /// $z$-component of $o$) by the unit rate of change of the $z$-component of 
    /// $d$ (which is the $z$-component of $d$). It then suffices to check if this
    /// point of intersection $r(t)$ with $z=0$ lies in the the square.
    fn intersect(&self, ray: &Ray3) -> ShapeIntersectionInfo {
        let transformed_ray = self.transform.ray_to_local(ray);

        if transformed_ray.direction.z().is_zero() {
            return ShapeIntersectionInfo::no_intersection();
        }

        let t = Float::abs(transformed_ray.origin.z()) / transformed_ray.direction.z();

        if !ray.is_in_range(t) {
            return ShapeIntersectionInfo::no_intersection();
        }

        let intersection_with_plane = {
            let mut temp = transformed_ray.eval(t);
            // project to account for floating-point error
            temp.set_z(0.0);
            temp
        };

        ShapeIntersectionInfo {
            did_hit: true,
            surface_normal: self.transform.vector_to_global(&Vec3::new(0.0,0.0,1.0)),
            t,
            point: self.transform.point_to_global(&intersection_with_plane),
            texture_coordinates: TextureCoordinates::default(),
        }
    }
}

impl Transformable for Quad {
    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }
}

impl ShapeLike for Quad {}

