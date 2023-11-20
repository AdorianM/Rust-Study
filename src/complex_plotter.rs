use plotters::prelude::*;

use crate::complex::Complex;

pub fn plot_complex(complex_numbers: &[Complex]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Complex plotter", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;

    chart.configure_mesh().draw()?;

    let complex_vec = complex_numbers.iter().map(| complex | (complex.r, complex.i) );
    

    root.present()?;

    Ok(())
}