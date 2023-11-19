use std::fmt::Display;

use crate::{Angle, complex::Complex};

pub struct Polar {
    pub magnitude: f64,
    pub angle: Angle
}

// TODO (after there is such a thing as exponentials in this code): Can be used as magnitude*e^(i*angle)
impl Polar {
    pub fn as_complex(self) -> Complex {
        // In a ◢ abc
        //    c
        //   /|
        //  / |
        // a--b
        // sin(a) = bc/ac (In our context, the imaginary part of the rectangular form) => i = sin(a) * ac
        // cos(a) = ab/ac (In our context, the real part of the rectangular form)      => r = cos(a) * ac
        let i = self.angle.as_radians().sin() * self.magnitude;
        let r = self.angle.as_radians().cos() * self.magnitude;

        Complex { r, i }
    }
}

impl Display for Polar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{:.precision$} ∢ {:.precision$}", self.magnitude, self.angle)
        } else {
            write!(f, "{} ∢ {}", self.magnitude, self.angle)
        }
    }
}