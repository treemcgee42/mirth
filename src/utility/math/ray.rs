use super::{vector::{Point3, Vec3}, float::{Float, FLOAT_ERR}};


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

