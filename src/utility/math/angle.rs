use super::float::Float;

pub enum AngleUnits {
    Degrees,
    Radians,
}

pub struct Angle {
    pub amount: Float,
    pub units: AngleUnits,
}

impl Angle {
    pub fn as_degrees(&self) -> Float {
        match self.units {
            AngleUnits::Degrees => self.amount,
            AngleUnits::Radians => Float::to_radians(self.amount),
        }
    }

    pub fn as_radians(&self) -> Float {
        match self.units {
            AngleUnits::Degrees => Float::to_degrees(self.amount),
            AngleUnits::Radians => self.amount,
        }
    }
}
