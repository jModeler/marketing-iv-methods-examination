use plotters::prelude::*;

pub fn plot_bias_vs_alpha_y(
    alpha_y_values: &Vec<f64>,
    bias_values: &Vec<f64>,
    file_name: &str,
    title: &str,
    x_label: &str,
    y_label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(1.0f64..3.0f64, -1.0f64..1.0f64)?;  // Make sure bounds are appropriate

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()?; // Now this is separated

    chart
        .draw_series(LineSeries::new(
            alpha_y_values.iter().zip(bias_values.iter()).map(|(x, y)| (*x, *y)),
            &RED,
        ))?
        .label("Bias")
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &RED));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?; // Optional: shows legend

    Ok(())
}
