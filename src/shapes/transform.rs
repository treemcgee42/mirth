use crate::utility::linalg::{Matrix4, Point3, Vec3, Ray3};

/// Conceptually, this struct is used to move between local and global coordinates.
#[derive(Clone)]
pub struct Transform {
    /// from local to global
    matrix: Matrix4,
    /// from global to local
    inverse_matrix: Matrix4,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            matrix: Matrix4::default(),
            inverse_matrix: Matrix4::default(),
        }
    }
}

impl Transform {
    pub fn point_to_local(&self, point: &Point3) -> Point3 {
        self.inverse_matrix.transform_point(point)
    }

    pub fn vector_to_local(&self, vector: &Vec3) -> Vec3 {
        self.inverse_matrix.transform_vector(vector)
    }

    pub fn ray_to_local(&self, ray: &Ray3) -> Ray3 {
        let mut to_return: Ray3 = ray.clone();
        to_return.origin = self.point_to_local(&ray.origin);
        to_return.direction = self.vector_to_local(&ray.direction);

        to_return
    }

    pub fn point_to_global(&self, point: &Point3) -> Point3 {
        self.matrix.transform_point(point)
    }

    pub fn vector_to_global(&self, vector: &Vec3) -> Vec3 {
        self.matrix.transform_vector(vector)
    }

    pub fn ray_to_global(&self, ray: &Ray3) -> Ray3 {
        let mut to_return: Ray3 = ray.clone();
        to_return.origin = self.point_to_global(&ray.origin);
        to_return.direction = self.vector_to_global(&ray.direction);

        to_return
    }
}

