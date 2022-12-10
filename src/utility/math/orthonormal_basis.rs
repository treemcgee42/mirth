use super::{
    vector::{Vec3, cross}, 
    float::Float
};

pub struct OrthonormalBasis {
    x_axis: Vec3,
    y_axis: Vec3,
    z_axis: Vec3,
}

impl OrthonormalBasis {
    /// Generates a new orthonormal basis given a single vector. There is no 
    /// canonical way to do this-- we do it as follows: the z-axis is the 
    /// (normalized) provided `v`...
    pub fn new_from_vector(v: &Vec3) -> Self {
        let z_axis = v.clone().normalize();

        // Approach from Shirley
        let a = {
            if Float::abs(z_axis.x()) > 0.9 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            }
        };

        let y_axis = cross(&z_axis, &a).normalize();
        let x_axis = cross(&z_axis, &y_axis);

        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    /// Takes a vector in local coordinates (wrt this orthonormal basis) and 
    /// returns the transformed vector in global coordinates.
    pub fn vector_from_local(&self, v: Vec3) -> Vec3 {
        (v.x() * &self.x_axis) + (v.y() * &self.y_axis) + (v.z() * &self.z_axis)
    }
}

