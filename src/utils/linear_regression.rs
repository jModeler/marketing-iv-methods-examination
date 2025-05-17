use linfa::prelude::*;
use linfa::dataset::Dataset;
use linfa_linear::{LinearRegression, FittedLinearRegression};
use ndarray::{Array2, Array1};

// Run a regression model
pub fn run_regression<X,Y>(
    x: &Array2<f64>,
    y: &Array2<f64>,
    intercept: bool,
) -> Result<FittedLinearRegression<f64>, String> {
    // Convert y from (n,1) to a flat array
    let y_flat: Array1<f64> = y.column(0).to_owned();

    // Create a Dataset from x and y
    let dataset = Dataset::new(x.to_owned(), y_flat);

    LinearRegression::default()
        .with_intercept(intercept)
        .fit(&dataset)
        .map_err(|e| format!("Failed to fit linear regression: {}", e))    
}