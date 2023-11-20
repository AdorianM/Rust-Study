#[allow(dead_code)]
mod complex;
mod angle;
mod polar;
use crate::complex::Complex;
use crate::angle::Angle;

mod complex_plotter;
use complex_plotter::*;

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

    let angle_one = Angle::from_degrees(30.0);
    let angle_two = Angle::from_degrees(60.0);

    println!("{}", angle_one);
    println!("{:.2}", angle_two);
    println!("{}", angle_one.as_degrees());
    println!("{}", angle_one.as_radians());

    println!("-----------------");

    let complex_to_be_polarized = Complex { r: 5.0, i: 3.0 };
    println!("To be polarized: {:.2}", complex_to_be_polarized);

    let my_polar = complex_to_be_polarized.as_polar();
    println!("Polar: {:.2}", my_polar);

    let complex_depolarized = my_polar.as_complex();
    println!("Depolarized: {:.2}", complex_depolarized);

    println!("Negated: {:.2}", -complex_to_be_polarized);

    println!("-----------------");

    let complex_to_pow = Complex { r: 3.0, i: 1.0};
    println!("Power: {:.2}", complex_to_pow.pow(3.0));
    println!("Root: {:.2}", complex_to_pow.root(2.0));

    let _ = plot_complex(&[complex_to_pow, complex_depolarized]);
}
