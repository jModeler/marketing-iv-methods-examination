use plotters::prelude::*;

pub fn plot_bias_vs_alpha_y(alpha_y_values: &Vec<f64>, bias_values: &Vec<f64>, file_name: &str, title: &str, x_label: &str, y_label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .build_cartesian_2d(1.0..3.0, -0.5..0.5)?
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            alpha_y_values.iter().zip(bias_values.iter()).map(|(x, y)| (*x, *y)),
            &RED,
        ))?
        .label("Bias")
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &RED));

    chart.configure_secondary_axes().draw()?;

    Ok(())
}