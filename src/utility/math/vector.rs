use cgmath::InnerSpace;
use serde::Deserialize;
use super::float::{Float, SignCheckable};
use std::ops;

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "PreVec3")]
pub struct Vec3 {
    pub(crate) internal: cgmath::Vector3<Float>,
}

// S==== SERDE {{{1

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

// E==== SERDE }}}1

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

// S==== OPERATOR OVERLOADS {{{1

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

// E==== OPERATOR OVERLOADS }}}1

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

