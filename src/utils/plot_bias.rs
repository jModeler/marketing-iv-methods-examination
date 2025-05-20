use plotters::prelude::*;

/// Plots the bias values against the alpha_y values and saves the plot to a file.
///
/// This function generates a 2D line plot where the x-axis represents the `alpha_y` values, 
/// and the y-axis represents the corresponding bias values. It then saves the plot as an image 
/// in the specified file.
///
/// # Parameters
/// 
/// - `x_vals`: A slice of `f64` values representing the `alpha_y` values to be plotted on the x-axis.
/// - `y_vals`: A slice of `f64` values representing the bias values to be plotted on the y-axis.
/// - `filename`: The path (including file name) where the plot image will be saved.
/// - `title`: The title of the plot.
/// - `x_label`: The label for the x-axis.
/// - `y_label`: The label for the y-axis.
///
/// # Returns
/// 
/// Returns a `Result<(), Box<dyn std::error::Error>>`, where `Ok(())` indicates success and
/// any error encountered during the plot generation (e.g., drawing, file writing) is returned as a `Box<dyn std::error::Error>`.
///
/// # Example
/// 
/// ```rust
/// use marketing_iv_methods::utils::plot_bias::plot_bias_vs_alpha_y; 
///
/// let alpha_y_values = vec![1.0, 1.5, 2.0, 2.5];
/// let bias_values = vec![0.2, 0.15, 0.1, 0.05];
/// let filename = "bias_vs_alpha_y.png";
/// let title = "Bias vs Alpha_y";
/// let x_label = "Alpha_y";
/// let y_label = "Bias";
/// plot_bias_vs_alpha_y(&alpha_y_values, &bias_values, filename, title, x_label, y_label).unwrap();
    /// ```
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
