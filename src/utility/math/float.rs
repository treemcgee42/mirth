
/// The type used for most global calculations. Should probably either be `f32` or `f64`.
pub type Float = f32;
pub static FLOAT_ERR: Float = 0.00001;

pub trait FloatConstants {
    fn get_pi() -> Self;
    fn get_1_pi() -> Self;
}

impl FloatConstants for f32 {
    fn get_pi() -> f32 {
        std::f32::consts::PI
    }

    fn get_1_pi() -> Self {
        std::f32::consts::FRAC_1_PI
    }
}

impl FloatConstants for f64 {
    fn get_pi() -> f64 {
        std::f64::consts::PI
    }

    fn get_1_pi() -> Self {
        std::f64::consts::FRAC_1_PI
    }
}

/// Idomatic way of querying which type of Float is being used. This is useful for things
/// like querying numbers from `RandomNumberGenerator`.
pub trait KindOfFloatCheckable {
    fn kind() -> KindOfFloat;
}

pub enum KindOfFloat {
    Float32,
    Float64,
}

impl KindOfFloatCheckable for f32 {
    fn kind() -> KindOfFloat {
        KindOfFloat::Float32
    }
}

impl KindOfFloatCheckable for f64 {
    fn kind() -> KindOfFloat {
        KindOfFloat::Float64
    }
}

pub trait SignCheckable {
    fn is_negative(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_zero(&self) -> bool;
}

impl SignCheckable for Float {
    fn is_negative(&self) -> bool {
        self < &((-1 as Float) * FLOAT_ERR)
    }

    fn is_positive(&self) -> bool {
        self > &FLOAT_ERR
    }

    fn is_zero(&self) -> bool {
        !(self.is_negative()) && !(self.is_positive())
    }
}

