//! Linear algebra stuff like vectors and matrices

use std::ops;

use crate::config::{Float, FLOAT_ERR, SignCheckable};
use image;
use cgmath::{self, Transform, InnerSpace, SquareMatrix};
use serde::Deserialize;

pub type Radians = Float;

pub type Color3 = Vec3;
impl From<Color3> for image::Rgb<f32> {
    fn from(color: Color3) -> Self {
        image::Rgb {
            0:[color.x().into(), color.y().into(), color.z().into()]
        }
    }
}

pub type Point3 = Vec3;

impl Point3 {
    pub fn origin() -> Self {
        Point3::new(0.0, 0.0, 0.0)
    }
}

// S==== VECTOR {{{1

// S==== SERDE {{{2

#[derive(Deserialize)]
struct PreVec3 {
    x: Float,
    y: Float,
    z: Float,
}

impl From<PreVec3> for Vec3 {
    fn from(pre_vec: PreVec3) -> Self {
        Vec3::new(pre_vec.x, pre_vec.y, pre_vec.z)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "PreVec3")]
pub struct Vec3 {
    internal: cgmath::Vector3<Float>,
}

// E==== SERDE }}}2

impl Vec3 {
    /// Create a new vector with the specified coordinates
    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 {
            internal: cgmath::Vector3::new(x, y, z),
        }
    }

    /// Retrieve the x coordinate.
    pub fn x(&self) -> Float { self.internal.x }
    /// Retrieve the y coordinate.
    pub fn y(&self) -> Float { self.internal.y }
    /// Retrieve the z coordinate.
    pub fn z(&self) -> Float { self.internal.z }

    /// Change the x coordinate to the specified value.
    pub fn set_x(&mut self, x: Float) { self.internal.x = x; }
    /// Change the y coordinate to the specified value.
    pub fn set_y(&mut self, y: Float) { self.internal.y = y; }
    /// Change the z coordinate to the specified value.
    pub fn set_z(&mut self, z: Float) { self.internal.z = z; }

    pub fn normalize(mut self) -> Self {
        self.internal = self.internal.normalize();
        self
    }

    pub fn normalize_to(mut self, magnitude: Float) -> Self {
        self.internal = self.internal.normalize_to(magnitude);
        self
    }

    pub fn are_equal(v1: &Vec3, v2: &Vec3) -> bool {
        (v1.x() - v2.x()).is_zero()
        && (v1.y() - v2.y()).is_zero()
        && (v1.z() - v2.z()).is_zero()
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> Float {
    cgmath::dot(v1.internal, v2.internal)
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3 {
        internal: v1.internal.cross(v2.internal),
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

// S==== OPERATOR OVERLOADS {{{2

// Vec3 + Vec3
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let v = self.internal + rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// Vec3 + &Vec3
impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        let v = self.internal + rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// &Vec3 + Vec3
impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let v = self.internal + rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// &Vec3 + &Vec3
impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        let v = self.internal + rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// Vec3 - Vec3
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let v = self.internal - rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// Vec3 - &Vec3
impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        let v = self.internal - rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// &Vec3 - Vec3
impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let v = self.internal - rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// &Vec3 - &Vec3
impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        let v = self.internal - rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// Float * Vec3
impl ops::Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

// Float * &Vec3
impl ops::Mul<&Vec3> for Float {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

// E==== OPERATOR OVERLOADS }}}2

// E==== VECTOR }}}1

// S==== RAY {{{1

#[derive(Clone, Debug, Default)]
pub struct Ray3 {
    pub origin: Point3,
    pub direction: Vec3,
    pub min_t: Float,
    pub max_t: Float,
}

impl Ray3 {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            min_t: FLOAT_ERR,
            max_t: Float::INFINITY,
        }
    }

    pub fn is_in_range(&self, t: Float) -> bool {
        (self.min_t < t) && (t < self.max_t)
    }

    pub fn eval(&self, t: Float) -> Point3 {
        &self.origin + t * &self.direction
    }
}

// E==== RAY }}}1

// S==== MATRIX {{{

#[derive(Clone, Debug)]
pub struct Matrix4 {
    internal: cgmath::Matrix4<Float>,
}

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4 { internal: cgmath::Matrix4::identity() }
    }
}

impl Matrix4 {
    pub fn transform_point(&self, point: &Point3) -> Point3 {
        let internal_point: cgmath::Point3<Float> = cgmath::point3(point.x(), point.y(), point.z());
        let xformed_point = self.internal.transform_point(internal_point);

        Point3::new(xformed_point.x, xformed_point.y, xformed_point.z)
    }

    pub fn transform_vector(&self, vector: &Vec3) -> Vec3 {
        let xformed_vec = self.internal.transform_vector(vector.internal);

        Vec3::new(xformed_vec.x, xformed_vec.y, xformed_vec.z)
    }
}

impl Matrix4 {
    /// Creates an affine transformation matrix with the provided vectors. 
    /// The last row of this matrix is set to $(0,0,0,1)$.
    pub fn new_from_column_vec3s(cols: [&Vec3; 4]) -> Self {
        let internal = cgmath::Matrix4::new(
            cols[0].x(), cols[0].y(), cols[0].z(), 0.0, // column 0
            cols[1].x(), cols[1].y(), cols[1].z(), 0.0, // column 1
            cols[2].x(), cols[2].y(), cols[2].z(), 0.0, // column 2
            cols[3].x(), cols[3].y(), cols[3].z(), 1.0, // column 3
        );

        Self {
            internal,
        }
    }

    /// Creates a transformation matrix describing rotation around `axis` by `angle` radians.
    pub fn new_from_axis_rotation_radians(axis: &Vec3, angle: Float) -> Self {
        let internal = cgmath::Matrix4::from_axis_angle(axis.internal, cgmath::Rad(angle));

        Self {
            internal
        }
    }

    /// Creates a transformation matrix describing rotation around `axis` by `angle` degrees.
    pub fn new_from_axis_rotation_degrees(axis: &Vec3, degrees: Float) -> Self {
        let radians: cgmath::Rad<Float> = cgmath::Deg(degrees).into();    
        Self::new_from_axis_rotation_radians(axis, radians.0)
    }

    pub fn inverse(&self) -> Self {
        let new_internal = self.internal.invert()
            .expect("tried to invert a noninvertible matrix");

        Matrix4 {
            internal: new_internal
        }
    }
}

// E==== MATRIX }}}

// S==== ORTHONORMAL BASIS {{{1 

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

// E==== ORTHONORMAL BASIS }}}1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_vec3_from_json() {
        #[derive(Deserialize)]
        struct TestWrapper {
            v: Vec3,
        }

        let json_str = r#"
            {
                "v": [0.0, 1.1, -3.2]
            }
        "#;

        let parsed: TestWrapper = serde_json::from_str(json_str).unwrap();
        assert!(Vec3::are_equal(&parsed.v, &Vec3::new(0.0,1.1,-3.2)));
    }
}

