use crate::utils::linear_regression::run_regression;
use super::generate_vector_data::{ind_var_generate, dep_var_generate};
use ndarray::{Array2, Axis, concatenate};
use linfa_linear::FittedLinearRegression;

pub fn run_yx_regression(n: usize, beta: f64, alpha_y: f64, alpha_x: f64, sigma_a: f64, sigma_ex: f64, sigma_ey: f64, intercept: bool) -> Result<FittedLinearRegression<f64>, String> {
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
    let yx_regression = match run_regression::<&Array2<f64>, &Array2<f64>>(&x, &dep_vars.y, intercept) { // Rust compiler could not infer the types for X and y, hence the explicit type annotations
        Ok(vars) => { vars }
        Err(err_msg) => {
            eprintln!("Error in the regression step: {}", err_msg);
            return Err("Error in the regression step".into());
        }
    };

    // return the regression result object
    Ok(yx_regression)
}
