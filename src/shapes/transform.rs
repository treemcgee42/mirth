use crate::utility::linalg::{Matrix4, Point3, Vec3, Ray3, cross};

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

// S==== TRANSFORMING OBJECTS {{{1

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

// E==== TRANSFORMING OBJECTS }}}1

// S==== CONSTRUCTORS {{{1

impl Transform {
    /// Produces a `Transform` given a matrix describing the conversion of local 
    /// coordinates to global coordinates.
    pub fn new_from_matrix(m: &Matrix4) -> Self {
        let matrix = m.clone();
        let inverse_matrix = matrix.inverse();

        Self {
            matrix,
            inverse_matrix
        }
    }

    /// Constructs a `Transform` in a way that something like a camera (a "viewer")
    /// would need. Local space will be local to the viewer.
    ///
    /// Paramaters:
    ///     - `look_from`: where the viewer is looking from (e.g. where it is centered)
    ///     - `look_at`: the point the viewer is looking at 
    ///     - `up_direction`: this is not exactly the the up direction for the transform,
    ///     but is a general guide, since we recompute it to ensure we have an orthonormal 
    ///     basis for the transform.
    pub fn new_for_viewer(look_from: &Point3, look_at: &Point3, up_direction: &Vec3) -> Self {
        // Mirth convention is +x is to the right, +y is up, and +z is into the screen
        let e2 = (look_from - look_at).normalize(); // +z
        let e0 = cross(up_direction, &e2).normalize(); // +x
        let e1 = cross(&e2, &e0); // +y
        let e3 = look_from; // translation

        let matrix = Matrix4::new_from_column_vec3s([&e0,&e1,&e2,e3]);
        let inverse_matrix = matrix.inverse();

        Self {
            matrix,
            inverse_matrix,
        }
    }
}

// E==== CONSTRUCTORS }}}1

