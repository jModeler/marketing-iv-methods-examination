use crate::utils::linear_regression::run_regression;
use super::generate_vector_data::{ind_var_generate, dep_var_generate};
use ndarray::{Array2, Axis, concatenate};
use linfa_linear::FittedLinearRegression;

/// A structure to hold the generated data used in the regression models.
///
/// This structure stores all the data generated for the regression process, including:
/// - `y`: The dependent variable `y`.
/// - `x`: The independent variable `x`.
/// - `v`: The independent variable `v`.
/// - `e_y`: The error term `e_y` for the dependent variable.
/// - `sigma_ex`: The standard deviation of the error term `e_x`.
/// - `sigma_a`: The standard deviation of the independent variable `v`.
/// - `alpha_x`: The coefficient of the independent variable `v`.
/// - `alpha_y`: The coefficient of the independent variable `x`.
///
/// # Example
///
/// ```rust
/// let generated_data = GeneratedData {
///     y: Array2::zeros((5, 1)),
///     x: Array2::zeros((5, 1)),
///     v: Array2::zeros((5, 1)),
///     e_y: Array2::zeros((5, 1)),
///     sigma_ex: 1.0,
///     sigma_a: 1.0,
///     alpha_x: 2.0,
///     alpha_y: 1.5,
/// };
/// ```
#[derive(Debug)]
pub struct GeneratedData {
    pub y: Array2<f64>,
    pub x: Array2<f64>,
    pub v: Array2<f64>,
    pub e_y: Array2<f64>,
    pub sigma_ex: f64,
    pub sigma_a: f64,
    pub alpha_x: f64,
    pub alpha_y: f64,
}

/// Runs a regression of `y` on `x` and `v`, and returns the fitted regression model along with the generated data.
///
/// This function generates independent and dependent variables using the given parameters, then runs a regression model of `y` on `x` and `v`. It returns both the fitted regression model and the generated data as a tuple.
///
/// # Parameters
/// 
/// - `params`: A tuple containing:
///   - `n`: The number of observations (number of rows in the generated arrays).
///   - `beta`: The coefficient for `x` in the dependent variable equation.
///   - `alpha_y`: The coefficient for `v` in the dependent variable equation.
///   - `alpha_x`: The coefficient for `v` in the independent variable equation.
///   - `sigma_a`: The standard deviation of the error term `v`.
///   - `sigma_ex`: The standard deviation of the error term `e_x`.
///   - `sigma_ey`: The standard deviation of the error term `e_y`.
///   - `intercept`: A boolean indicating whether to include an intercept in the regression.
///
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok`: A tuple where the first element is the fitted `FittedLinearRegression<f64>` model and the second is the `GeneratedData` struct.
/// - `Err`: An error message if any of the data generation or regression steps fail.
///
/// # Example
///
/// ```rust
/// let params = (100, 0.5, 1.0, 2.0, 1.0, 0.5, 1.0, true);
/// let result = run_yxv_regression(params);
/// match result {
///     Ok((model, data)) => {
///         println!("{:?}", model);
///         println!("{:?}", data);
///     }
///     Err(err) => {
///         println!("Error: {}", err);
///     }
/// }
/// ```
pub fn run_yxv_regression(params: (usize, f64, f64, f64, f64, f64, f64, bool)) -> Result<(FittedLinearRegression<f64>, GeneratedData), String> {
    let (n, beta, alpha_y, alpha_x, sigma_a, sigma_ex, sigma_ey, intercept) = params;

    let ind_vars = match ind_var_generate(n, alpha_x, sigma_a, sigma_ex) {
        Ok(vars) => { vars }
        Err(err_msg) => {
            eprintln!("Error generating independent variables: {}", err_msg);
            return Err("Error generating independent variables".into());
        }
    };
    
    let dep_vars = match dep_var_generate(beta, alpha_y, sigma_ey, ind_vars) {
        Ok(vars) => { vars }
        Err(err_msg) => {
            eprintln!("Error generating dependent variables: {}", err_msg);
            return Err("Error generating dependent variables".into());
        }
    };
    
    // create input data array
    let x = concatenate(Axis(1), &[dep_vars.ind_vars.x.view(), dep_vars.ind_vars.v.view()]).unwrap();

    // run the regression
    let yxv_regression = match run_regression(&x, &dep_vars.y, intercept) {
        Ok(vars) => { vars }
        Err(err_msg) => {
            eprintln!("Error in the regression step: {}", err_msg);
            return Err("Error in the regression step".into());
        }
    };

    // create a tuple that contains the data I'll need for later regressions
    let generated_data = GeneratedData {
        y: dep_vars.y,
        x: dep_vars.ind_vars.x,
        v: dep_vars.ind_vars.v,
        e_y: dep_vars.e_y,
        sigma_ex: dep_vars.ind_vars.sigma_ex,
        sigma_a: dep_vars.ind_vars.sigma_a,
        alpha_x: dep_vars.ind_vars.alpha_x,
        alpha_y: dep_vars.alpha_y,
    };

    // return the tuple of the regression result and the tuple of generated data
    Ok((yxv_regression, generated_data))
}

/// Runs additional regression models, including regression of `y` on `x`, and regression of the composite error term (`alpha_y * v + e_y`) on `x`.
/// It also calculates the bias term using a formula from the Rossi paper.
///
/// # Parameters
///
/// - `generated_data`: A reference to a `GeneratedData` struct containing the generated data for regression.
/// - `intercept`: A boolean indicating whether to include an intercept in the regression.
///
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok`: A tuple where the first element is the regression of `y` on `x`, the second element is the regression of the composite error term on `x`,
///         and the third element is the bias term calculated using a formula from the Rossi paper.
/// - `Err`: An error message if any of the regression steps fail.
///
/// # Example
///
/// ```rust
/// let generated_data = GeneratedData {
///     y: Array2::zeros((5, 1)),
///     x: Array2::zeros((5, 1)),
///     v: Array2::zeros((5, 1)),
///     e_y: Array2::zeros((5, 1)),
///     sigma_ex: 1.0,
///     sigma_a: 1.0,
///     alpha_x: 2.0,
///     alpha_y: 1.5,
/// };
/// let intercept = true;
/// let result = run_other_regressions(&generated_data, intercept);
/// match result {
///     Ok((yx_regression, vex_regression, bias)) => {
///         println!("{:?}", yx_regression);
///         println!("{:?}", vex_regression);
///         println!("{}", bias);
///     }
///     Err(err) => {
///         println!("Error: {}", err);
///     }
/// }
/// ```
pub fn run_other_regressions(generated_data: &GeneratedData, intercept: bool) -> Result<(FittedLinearRegression<f64>, FittedLinearRegression<f64>, f64), String> {

    // run the regression of y on x alone
    let yx_regression = match run_regression(&generated_data.x, &generated_data.y, intercept) {
        Ok(model) => { model }
        Err(err_msg) => {
            eprintln!("Error in the regression of y on x: {}", err_msg);
            return Err("Error in the regression step of y on x".into());
        }
    };

    // generate composite error term
    let ve = generated_data.alpha_y * &generated_data.v + &generated_data.e_y;

    // run the regression of alpha_y*v + e_y on x alone
    let vex_regression = match run_regression(&generated_data.x, &ve, intercept) {
        Ok(model) => { model }
        Err(err_msg) => {
            eprintln!("Error in the regression of composite error term on x: {}", err_msg);
            return Err("Error in the regression of composite error term on x".into());            
        }
    };

    // calculate the bias term from the formula provided in the Rossi paper
    let bias = generated_data.alpha_y * generated_data.alpha_x * generated_data.sigma_a.powf(2.0)/(generated_data.alpha_x.powf(2.0) * generated_data.sigma_a.powf(2.0) + generated_data.sigma_ex.powf(2.0));

    // return results
    Ok((yx_regression, vex_regression, bias))
}