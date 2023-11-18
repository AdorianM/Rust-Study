use core::fmt;
use std::ops::Add;
use std::fmt::Display;

use crate::angle::Angle;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Complex {
    pub r: f64,
    pub i: f64
}

impl Complex {
    pub fn conjugate(self) -> Complex {
        Complex { r: self.r, i: -self.i }
    }

    pub fn magnitude(self) -> f64 {
        (self.r * self.r + self.i * self.i).sqrt()
    }

    pub fn argument(self) -> Angle {
        Angle { value: self.i.atan2(self.r), unit: crate::angle::AngleUnit::Radians }
    }
}

impl std::ops::Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> <Self as Add<Complex>>::Output {
        Complex {r: self.r + other.r, i: self.i + other.i}
    }
}

impl std::ops::Sub<Complex> for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> <Self as Add<Complex>>::Output {
        Complex {r: self.r - other.r, i: self.i - other.i}
    }
}

impl std::ops::Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Self::Output {
        let r = self.r * other.r - self.i * other.i;
        let i = self.r * other.i + self.i * other.r;
        Complex { r, i }
    }
}


impl std::ops::Div<Complex> for Complex {
    type Output = Complex;
    fn div(self, other: Complex) -> Self::Output {
        // Division result can be organized in real and imaginary part
        // by multiplying the denominator by its conjugate
        let r_numerator = self.r * other.r + self.i * other.i;
        let i_numerator = self.i * other.r - self.r * other.i;
        let common_denominator = other.r * other.r + other.i * other.i;

        let r = r_numerator / common_denominator;
        let i = i_numerator / common_denominator;
        Complex { r, i }
    }
}
impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(precision) = f.precision() {
            match (self.r, self.i) {
                (x, _) if x == 0.0 => write!(f, "{:.precision$}i", self.i, precision = precision),
                (_, y) if y == 0.0 => write!(f, "{:.precision$}", self.r, precision = precision),
                (x, y) if x.is_nan() || y.is_nan() => write!(f, "{}", x),
                (_, _) => {
                    let sign = if self.i > 0.0 {
                        "+"
                    } else {
                        "-"
                    };
    
                    write!(f, "{:.precision$} {} {:.precision$}", self.r, sign, self.i.abs(), precision = precision)
                }
            }
        } else {
            match (self.r, self.i) {
                (x, _) if x == 0.0 => write!(f, "{}i", self.i),
                (_, y) if y == 0.0 => write!(f, "{}", self.r),
                (x, y) if x.is_nan() || y.is_nan() => write!(f, "{}", x),
                (_, _) => {
                    let sign = if self.i > 0.0 {
                        "+"
                    } else {
                        "-"
                    };
    
                    write!(f, "{} {} {}", self.r, sign, self.i.abs())
                }
            }
        }

    }
}