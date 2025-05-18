use crate::utils::linear_regression::run_regression;
use super::generate_vector_data::{ind_var_generate, dep_var_generate};
use ndarray::{Array2, Axis, concatenate};
use linfa_linear::FittedLinearRegression;

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

// pub fn run_other_regressions(generated_data: GeneratedData) -> Result<(FittedLinearRegression<f64>, FittedLinearRegression<f64>), String> {
//     // run the regression of y on x alone
//     let (yx_)
// }