use crate::utility::math::{vector::Color3, float::Float};


pub type Light = Color3;

enum KindOfLight {
    RGB,
}

trait KindOfLightCheckable {
    fn kind_of_light() -> KindOfLight;
}

impl KindOfLightCheckable for Color3 {
    fn kind_of_light() -> KindOfLight {
        KindOfLight::RGB
    }
}

pub type Spectrum = Color3;

pub trait ConvertableToLight {
    fn to_light(&self, wavelength: Float) -> Light;
}

impl ConvertableToLight for Color3 {
    fn to_light(&self, wavelength: Float) -> Light {
        match Light::kind_of_light() {
            KindOfLight::RGB => {
                return self.clone();
            }
        }
    }
}

pub trait ColorConstantsQueryable {
    fn black() -> Self;
    fn white() -> Self;
}

impl ColorConstantsQueryable for Color3 {
    fn black() -> Self {
        Color3::new(0.0, 0.0, 0.0)
    }

    fn white() -> Self {
        Color3::new(1.0, 1.0, 1.0)
    }
}

