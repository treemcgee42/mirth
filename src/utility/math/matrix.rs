use cgmath::{SquareMatrix, Transform};

use super::{float::Float, vector::{Point3, Vec3}, angle::{Angle, AngleUnits}};


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

// S==== CONSTRUCTING TRANSFORMATIONS {{{2

pub enum Matrix4TransformKind {
    AxisRotation(Matrix4AxisRotationInfo),
    Translation(Vec3),
    Scale(Vec3),
}

pub struct Matrix4AxisRotationInfo {
    pub axis: Vec3,
    pub angle: Angle,
}

impl Matrix4 {
    pub fn new_from_sequence(sequence: &Vec<Matrix4TransformKind>) -> Self {
        let mut to_return = Matrix4::identity();

        for transform_kind in sequence.iter() {
            let single_transform_matrix = Self::new_from_transform_kind(transform_kind);
            to_return = single_transform_matrix * to_return;
        }

        to_return
    }
    
    fn new_from_transform_kind(kind: &Matrix4TransformKind) -> Self {
        match kind {
            Matrix4TransformKind::AxisRotation(rotation_info) => {
                Self::new_from_axis_rotation(rotation_info)
            },
            Matrix4TransformKind::Translation(translation) => {
                Self::new_from_translation(translation)
            },
            Matrix4TransformKind::Scale(scale) => {
                Self::new_from_scale(scale)
            }
        }
    }

    /// Creates a transformation matrix describing rotation around an axis.
    pub fn new_from_axis_rotation(info: &Matrix4AxisRotationInfo) -> Self {
        let axis = info.axis.internal;

        let internal: cgmath::Matrix4<Float>;
        match info.angle.units {
            AngleUnits::Degrees => {
                let angle = cgmath::Deg(info.angle.amount);
                internal = cgmath::Matrix4::from_axis_angle(axis, angle);
            }
            AngleUnits::Radians => {
                let angle = cgmath::Rad(info.angle.amount);
                internal = cgmath::Matrix4::from_axis_angle(axis, angle);
            }
        }

        Self {
            internal
        }
    }

    /// Creates a transformation matrix describing the translation by `translation` (in 
    /// homogeneous coordinates, (translation, 1)).
    pub fn new_from_translation(translation: &Vec3) -> Self {
        let internal = cgmath::Matrix4::from_translation(
            cgmath::Vector3 { x: translation.x(), y: translation.y(), z: translation.z() }
        );

        Self {
            internal,
        }
    }
    
    /// Creates a transformation matrix describing the componentwise scale `scale`. That is, 
    /// we scale the $x$-coordinate by `scale.x()`, etc.
    pub fn new_from_scale(scale: &Vec3) -> Self {
        let internal = cgmath::Matrix4::from_nonuniform_scale(scale.x(), scale.y(), scale.z());

        Self {
            internal,
        }
    }
}

// E==== CONSTRUCTING TRANSFORMATIONS }}}2

impl Matrix4 {
    pub fn identity() -> Self {
        Matrix4 { internal: cgmath::Matrix4::identity() }
    }

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

    

    pub fn inverse(&self) -> Self {
        let new_internal = self.internal.invert()
            .expect("tried to invert a noninvertible matrix");

        Matrix4 {
            internal: new_internal
        }
    }
}

impl std::ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let internal = self.internal * rhs.internal;
        Matrix4 {
            internal
        }
    }
}

