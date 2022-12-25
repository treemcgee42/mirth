
// S==== IMPORTS {{{1

use crate::utility::math::{
    vector::{Point3, dot, Vec3}, 
    ray::Ray3, 
    float::{Float, SignCheckable}
};
use super::{
    traits::{ShapeIntersectionInfo, IntersectableShape, Transformable, ShapeLike}, 
    transform::{Transform, self}
};

// E==== IMPORTS }}}1

pub struct Sphere {
    center: Point3,
    radius: Float,
    /// In the sphere's local coordinates, it is centered at the origin and has radius 1
    transform: Transform,
}

pub struct SphereInfo {
    pub center: Point3,
    pub radius: Float,
    pub transform: Transform,
}

impl Sphere {
    pub fn new(info: SphereInfo) -> Self {
        Self {
            center: info.center,
            radius: info.radius,
            transform: info.transform,
        }
    }
}

impl IntersectableShape for Sphere { // {{{1
    fn intersect(&self, ray: &Ray3) -> ShapeIntersectionInfo {
        // If the ray has equation o + td and sphere is centered at C with radius r, then if the
        // ray intersects it at the values of t such that At^2 + Bt + C = 0, where 
        // A = dot(d, d), B = 2 * dot(d, o-c), C = dot(o-c, o-c) - r^2. There is no intersection
        // when there are no real values of t.
        
        // In general, the transformed sphere may not be a sphere, so the above computation
        // is not valid in camera coordinates, which is what the ray is expressed in. Instead,
        // we can transform the ray into the sphere's coordinates, where the sphere is centered
        // at the origin.
        
        let mut to_return = ShapeIntersectionInfo::default();
 
        let local_ray = self.transform.ray_to_local(ray);
        let o = &local_ray.origin;
        let d = &local_ray.direction;
        let center = &self.center;
        let r = self.radius;

        let a = dot(d, d);
        let b = (2 as Float) * dot(d, &(o-center));
        let c = dot(&(o-center), &(o-center)) - (r*r);

        let discriminant: Float = {
            let temp = (b*b) - ((4 as Float) * a * c);
            if temp.is_zero() { 0.0 as Float }
            else { temp as Float }
        };

        if discriminant.is_negative() {
            return ShapeIntersectionInfo::no_intersection();
        }

        let t: Float = {
            // take the smallest value of t (= closest intersection point)
            let t0 = ((-1 as Float) * b - Float::sqrt(discriminant)) / ((2 as Float) * a);

            if !local_ray.is_in_range(t0) {
                // an example of when the other root is important is if we have a glass ball, 
                // and are working with a camera ray that scattered inwards 
                let t1 = ((-1 as Float) * b + Float::sqrt(discriminant)) / ((2 as Float) * a);
                if !local_ray.is_in_range(t1) { return ShapeIntersectionInfo::no_intersection(); }
                t1
            }
            else { t0 }
        };

        // Collect all calculations into return struct

        to_return.did_hit = true;
        to_return.point = {
            // Project hit point to sphere surface to account for floating point errors
            let pre_local_hitpoint = local_ray.eval(t);
            let local_hitpoint: Point3 = 
                &self.center + (pre_local_hitpoint - &self.center).normalize_to(self.radius);

            // Convert from local coordinates 
            self.transform.point_to_global(&local_hitpoint)
        };
        to_return.t = t;

        return to_return;
    }
} // }}}1

impl Transformable for Sphere {
    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }
}

impl ShapeLike for Sphere {}

// #[cfg(test)] // {{{1
// mod tests {
//     use crate::utility::math::vector::Vec3;
//
//     use super::*;
//
//     #[test]
//     fn no_transform_intersection() {
//         // Sphere: radius 2, centered at (0,0,3)
//         let sphere = Sphere::new(Point3::new(0.0,0.0,3.0), 2.0);
//
//         // Ray 1: (0,0,0) + t(0,0,1)            -- h1=(0,0,1), h2=(0,0,5)
//         let mut r1 = Ray3::new(
//             Point3::new(0.0, 0.0, 0.0), 
//             Vec3::new(0.0, 0.0, 1.0)
//         );
//         let mut r1_hitinfo = sphere.intersect(&r1);
//         assert!(
//             Point3::are_equal(&r1_hitinfo.point, &Vec3::new(0.0, 0.0, 1.0))
//         );
//
//         r1.min_t = 1.5;
//         r1_hitinfo = sphere.intersect(&r1);
//         assert!(
//             Point3::are_equal(&r1_hitinfo.point, &Vec3::new(0.0, 0.0, 5.0))
//         );
//
//         // Ray 2: (2,0,0) + t(0,0,0.3)          -- h1=(2,0,3), h2=n/a
//         let mut r2 = Ray3::new(
//             Point3::new(2.0, 0.0, 0.0),
//             Vec3::new(0.0, 0.0, 0.3)
//         );
//         let mut r2_hitinfo = sphere.intersect(&r2);
//         assert!(
//             Point3::are_equal(&r2_hitinfo.point, &Vec3::new(2.0, 0.0, 3.0)),
//         );
//
//         r2.min_t = 10.1;
//         r2_hitinfo = sphere.intersect(&r2);
//         assert!(r2_hitinfo.did_hit == false);
//
//         // Ray 3: (0,0,0) + t(5,5,5)            -- h1=n/a, h2=n/a
//         let r3 = Ray3::new(
//             Point3::new(0.0, 0.0, 0.0),
//             Vec3::new(-1.0, -1.0, -1.0)
//         );
//         let r3_hitinfo = sphere.intersect(&r3);
//         assert!(r3_hitinfo.did_hit == false);
//     }
// } // }}}1

