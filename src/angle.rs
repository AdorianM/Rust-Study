use std::{f64::consts::PI, fmt::Display};

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Angle {
    pub(crate) value: f64,
    pub(crate) unit: AngleUnit
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum AngleUnit {
    Radians,
    Degrees
}

impl Angle {
    fn new(value: f64, unit: AngleUnit) -> Self {
        Self {value, unit}
    }

    pub fn from_radians(self, value: f64) -> Self {
        Angle::new(value, AngleUnit::Radians)
    }

    pub fn from_degrees(self, value: f64) -> Self {
        Angle::new(value, AngleUnit::Degrees) 
    }

    pub fn as_radians(self) -> f64 {
        match self.unit {
            AngleUnit::Radians => self.value,
            AngleUnit::Degrees => self.value * PI / 180.0,
        }
    }

    pub fn as_degrees(self) -> f64 {
        match self.unit {
            AngleUnit::Radians => self.value * 180.0 / PI,
            AngleUnit::Degrees => self.value,
        }
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            match self.unit {
                AngleUnit::Degrees => write!(f, "{:.precision$}°", self.value, precision = precision),
                AngleUnit::Radians => write!(f, "{:.precision$}", self.value, precision = precision)
            }
        } else {
            match self.unit {
                AngleUnit::Degrees => write!(f, "{}°", self.value),
                AngleUnit::Radians => write!(f, "{}", self.value)
            }
        }
    }
}