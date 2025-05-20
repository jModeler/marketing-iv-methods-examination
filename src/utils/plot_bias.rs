use plotters::prelude::*;

pub fn plot_bias_vs_alpha_y(
    x_vals: &[f64],
    y_vals: &[f64],
    filename: &str,
    title: &str,
    x_label: &str,
    y_label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = *x_vals.first().unwrap()..*x_vals.last().unwrap();
    let y_range = y_vals.iter().cloned().fold(f64::INFINITY..f64::NEG_INFINITY, |acc, y| {
        acc.start.min(y)..acc.end.max(y)
    });

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_range.clone(), y_range.clone())?;

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .disable_mesh() 
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            x_vals.iter().cloned().zip(y_vals.iter().cloned()),
            &RED,
        ))?
        .label("Bias")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?; // ✅ renders the legend

    Ok(())
}
