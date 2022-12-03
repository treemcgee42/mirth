//! Methods for generating random samples (points) on various geometries.

use crate::{utility::{rng::RandomNumberGenerator, linalg::Vec3}, config::{FloatConstants, Float}};



pub fn uniform_on_2sphere_hemisphere(rng: &mut RandomNumberGenerator) -> Vec3 {
    sphere_sampler_helper(rng, SphereSampleKind::UniformHemisphere)
}

pub fn cosine_on_2sphere_hemisphere(rng: &mut RandomNumberGenerator) -> Vec3 {
    sphere_sampler_helper(rng, SphereSampleKind::CosineHemisphere)
}

// S==== HELPERS {{{1

enum SphereSampleKind {
    /// upper hemisphere, uniformly
    UniformHemisphere,
    // upper hemisphere, uniform wrt cosine / solid angle (higher distribution near top)
    CosineHemisphere,
}

/// Helper encapsulating various ways to sample on the unit sphere.
fn sphere_sampler_helper(rng: &mut RandomNumberGenerator, kind: SphereSampleKind) -> Vec3 {
    // We sample spherical coordinates.

    let phi = (2 as Float) * Float::get_pi() * rng.next_float();
    let (sin_phi, cos_phi) = Float::sin_cos(phi);

    let cos_theta = match kind {
        SphereSampleKind::UniformHemisphere => {
            // By the Archimedes hat-box theorem, it suffices to sample the enscribing
            // cylinder.
            rng.next_float()
        },
        SphereSampleKind::CosineHemisphere => {
            Float::sqrt(rng.next_float())
        }
    };
    let sin_theta = Float::sqrt(1.0 - cos_theta * cos_theta);
    
    Vec3::new(
        cos_phi * sin_theta,
        sin_phi * sin_theta,
        cos_theta
    )
}

// E==== HELPERS }}}1

// S==== TESTS {{{1

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn gen_points_sphere_hemisphere() {
        let mut file = File::create("test-output/points.csv").expect("failed to create file");
        write!(&mut file, "x,y,z\n").unwrap();

        let mut rng = RandomNumberGenerator::from_seed(1);

        for _ in 0..1000 {
            let point = uniform_on_2sphere_hemisphere(&mut rng);
            write!(&mut file, "{},{},{}\n", point.x(), point.y(), point.z()).unwrap();
        }
    }

    #[test]
    fn gen_points_cosine_sphere_hemisphere() {
        let mut file = File::create("test-output/points.csv").expect("failed to create file");
        write!(&mut file, "x,y,z\n").unwrap();

        let mut rng = RandomNumberGenerator::from_seed(1);

        for _ in 0..1000 {
            let point = cosine_on_2sphere_hemisphere(&mut rng);
            write!(&mut file, "{},{},{}\n", point.x(), point.y(), point.z()).unwrap();
        }
    }
}

// E==== TESTS }}}1
