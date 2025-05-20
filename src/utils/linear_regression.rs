use linfa::prelude::*;
use linfa::dataset::Dataset;
use linfa_linear::{LinearRegression, FittedLinearRegression};
use ndarray::{Array2, Array1};

/// Runs a linear regression model using the provided feature matrix `x` and response vector `y`.
///
/// # Arguments
///
/// * `x` - A 2D array (`Array2<f64>`) representing the design matrix (independent variables).
/// * `y` - A 2D array (`Array2<f64>`) with a single column, representing the dependent variable.
/// * `intercept` - A boolean indicating whether to include an intercept term in the model.
///
/// # Returns
///
/// * `Ok(FittedLinearRegression<f64>)` if the model fits successfully.
/// * `Err(String)` if the regression fitting process fails.
///
/// # Example
///
/// ```
/// use ndarray::Array2;
/// use linfa_linear::FittedLinearRegression;
/// use marketing_iv_methods::utils::linear_regression::run_regression;
///
/// let y: Array2<f64> = Array2::from_elem((10, 1), 1.0);
/// let x: Array2<f64> = Array2::from_elem((10, 1), 0.5);
/// let result = run_regression(&x, &y, false).unwrap();
/// let coefficients = result.params();
/// ```
pub fn run_regression(
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