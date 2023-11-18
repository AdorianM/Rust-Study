mod complex;
mod angle;
use crate::complex::Complex;
use crate::angle::{Angle, AngleUnit};

fn main() {
    let number_one = Complex { r: 5.0, i: 10.0 };
    let number_two = Complex { r: 3.0, i: 7.0 };
    let number_zero= Complex { r: 0.0, i: 0.0 };

    println!("{}", number_one);
    println!("{}", number_one + number_two);
    println!("{}", number_one - number_two);
    println!("{}", number_one.conjugate());
    println!("{:.2}", number_one.magnitude());
    println!("{:.2}", number_one * number_two);
    println!("{:.5}", number_one / number_two);

    println!("Arguments: ");
    println!("{:.2}", number_one.argument());
    println!("{:.2}", number_two.argument());
    println!("{:.2}", number_zero.argument());

    let angle_one = Angle { value: 60.0, unit: AngleUnit::Degrees };
    let angle_two = Angle { value: 1.0, unit: AngleUnit::Radians };

    println!("{}", angle_one);
    println!("{:.2}", angle_two);
    println!("{}", angle_one.as_degrees());
    println!("{}", angle_one.as_radians());
    
}
