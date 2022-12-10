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

