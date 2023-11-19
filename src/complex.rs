use core::fmt;
use std::num;
use std::ops::Add;
use std::fmt::Display;

use crate::angle::Angle;
use crate::polar::Polar;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Complex {
    pub r: f64,
    pub i: f64
}

impl Complex {

    pub fn as_polar(self) -> Polar {
        Polar { magnitude: self.magnitude(), angle: self.argument() }
    }

    pub fn from_euler(angle: Angle) -> Complex {
        // Euler's Formula -> e^(ix) = cos(x) + i*sin(x)
        // e^(i*angle) = cos(angle) + i*sin(angle)

        let r = angle.as_radians().cos();
        let i = angle.as_radians().sin();

        Complex { r, i }
    }
    pub fn conjugate(self) -> Complex {
        Complex { r: self.r, i: -self.i }
    }

    pub fn magnitude(self) -> f64 {
        (self.r * self.r + self.i * self.i).sqrt()
    }

    pub fn argument(self) -> Angle {
        let angle_value = self.i.atan2(self.r); 
        Angle::from_radians(angle_value);
        Angle { value: self.i.atan2(self.r), unit: crate::angle::AngleUnit::Radians }
    }

    pub fn pow(&self, exponent: f64) -> Complex {
        // a + i*b = r * (cos(θ) + i*sin(θ))
        // r = sqrt(a^2 + b^2)
        // =>
        // (a + i*b)^n = r^n * (cos(nθ) + i*sin(nθ))
        let polar_form = self.as_polar();
        let new_magnitude = polar_form.magnitude.powf(exponent);
        let new_angle = Angle::from_radians(polar_form.angle.as_radians() * exponent);

        Polar { magnitude: new_magnitude, angle: new_angle }.as_complex()
    }

    pub fn root(&self, order: f64) -> Complex {
        self.pow(1.0/order)
    } 

    // If for any reason I decide to mix this with the actual num::complex::Complex implementation

    pub fn as_std_complex(self) {
        todo!()
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

impl std::ops::Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Complex { r: -self.r, i: -self.i }
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
    
                    write!(f, "{:.precision$} {} {:.precision$}i", self.r, sign, self.i.abs(), precision = precision)
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