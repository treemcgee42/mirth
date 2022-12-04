use crate::{utility::linalg::{Matrix4, Point3, Vec3, Ray3, cross}, config::Float};
use serde::Deserialize;
use tracing::warn;

/// Conceptually, this struct is used to move between local and global coordinates.
#[derive(Clone, Debug)]
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

// S==== PARSING {{{2

/// Only used to parse from json using `serde`
#[derive(Deserialize)]
struct PreViewerTransform {
    look_from: Vec3,
    look_at: Vec3,
    up_direction: Vec3,
}

#[derive(Deserialize)]
struct SimpleRotation {
    axis: Vec3,
    angle: Float,
}

impl Transform {
    /// For parsing from the scene file, the value of the field "transform". There are 
    /// several ways to specify a `Transform` in json:
    ///
    /// # viewer type
    /// 
    /// Corresponds to the construction via `new_for_viewer()`, and can be specified 
    /// as follows:
    /// ```
    /// {
    ///     "viewer: {
    ///         "look_from": Vec3,
    ///         "look_at": Vec3,
    ///         "up_direction": Vec3
    ///     }
    /// }
    /// ```
    /// All three fields must be specified.
    ///
    /// # simple sequence type
    /// 
    /// This is specified as a map of simple types, which are described below. It is 
    /// specified like
    /// ```
    /// {
    ///     "simple sequence": {
    ///         Simple1,
    ///         Simple2,
    ///         ...
    ///     }
    /// }
    /// `
    /// 
    /// The following are the simple types:
    ///
    /// ## rotation
    /// ```
    /// {
    ///     "rotation": {
    ///         "axis": Vec3,
    ///         "angle": Float
    ///     }
    /// }
    /// ```
    /// Here, the angle is specified in degrees.
    /// 
    pub fn new_from_json(json: &serde_json::Value) -> Result<Self,()> {
        if let Ok(transform) = Self::new_for_viewer_from_json(&json["viewer"]) {
            return Ok(transform);
        }

        Err(())
    }

    /// Tries to parse the json assuming it is a "viewer" type, i.e. something that could 
    /// be construction using `new_for_viewer`. 
    ///
    /// An error result means that either the json was not a viewer type, or was a viewer 
    /// type but was written incorrectly. TODO: we should handle the latter case better.
    fn new_for_viewer_from_json(json: &serde_json::Value) -> Result<Self, ()> {
        let to_return: Result<PreViewerTransform, _> = serde_json::from_value(json.clone());

        if let Ok(pvt) = to_return {
            Ok(Transform::new_for_viewer(
                &pvt.look_from, 
                &pvt.look_at,
                &pvt.up_direction
            ))
        } else {
            Err(())
        }
    }
}

// E==== PARSING }}}2

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_viewer_transform() { 
        let json_str = r#"
            {
                "transform": {
                    "look_from": [0,0,0],
                    "look_at": [1,1,-1],
                    "up_direction": [0,1,0]
                }
            }
        "#;

        let parsed_value: serde_json::Value = serde_json::from_str(json_str).unwrap();
    
        let transform = Transform::new_from_json(&parsed_value["transform"]).unwrap();
        println!("{:?}", transform);
    }
}

