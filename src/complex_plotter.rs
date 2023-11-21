use plotters::prelude::*;

use crate::complex::Complex;

pub fn complex_to_plottable(complex: Complex) -> (f32, f32) {
    (complex.r as f32, complex.i as f32)
}

pub fn scale_coords_to_grid(coordinates: &Vec<(f32, f32)>, x_range: (f32, f32), y_range: (f32, f32)) -> Vec<(f32, f32)>{
    assert!(x_range.0 < x_range.1);
    assert!(y_range.0 < y_range.1);
    let x_axis: Vec<f32> = coordinates.iter().map(|&c| c.0).collect();
    let y_axis: Vec<f32> = coordinates.iter().map(|&c| c.1).collect();

    let x_axis_scaled = scale_numbers_to_range(&x_axis, x_range.0, x_range.1);
    let y_axis_scaled = scale_numbers_to_range(&y_axis, y_range.0, y_range.1);

    x_axis_scaled.iter().zip(y_axis_scaled.iter()).map(
        |(&a, &b)| (a, b)
    ).collect()
}

pub fn scale_numbers_to_range(numbers: &Vec<f32>, range_min: f32, range_max: f32) -> Vec<f32> {
    assert!(range_min < range_max);

    let mut number_min: f32 = numbers[0];
    let mut number_max: f32 = numbers[0];

    for &number in numbers.iter() {
        if number < number_min {
            number_min = number;
        }
        if number > number_max {
            number_max = number;
        }
    }

    let range_size = range_max - range_min;
    let numbers_range_size = number_max - number_min;
    
    numbers.iter().map( |number| range_size * (number - number_min)/numbers_range_size + range_min).collect()
}

pub fn plot_complex_normalized(complex_numbers: &[Complex], cartesian_x_range: (f32, f32), cartesian_y_range: (f32, f32)) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Complex plotter", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(cartesian_x_range.0..cartesian_x_range.1, cartesian_y_range.0..cartesian_y_range.1)?;

    chart.configure_mesh().draw()?;

    let complex_coords_vec: Vec<(f32, f32)> = complex_numbers
        .iter()
        .map(| &complex | complex_to_plottable(complex) )
        .collect();

    let scaled_coords: Vec<(f32, f32)> = scale_coords_to_grid(&complex_coords_vec, cartesian_x_range, cartesian_y_range);

    chart.draw_series(PointSeries::of_element(
        scaled_coords.iter().cloned(),
        5,
        &RED,
        &|c, s, st| Circle::new(c, s, st.filled())
    ))?;

    root.present()?;

    Ok(())
}