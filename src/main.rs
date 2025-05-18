mod utils;
mod simple_example;

use ndarray::s;
use simple_example::generate_vector_data::{ind_var_generate, dep_var_generate};
use simple_example::run_regressions::{run_yxv_regression, run_other_regressions};


fn main() {
    // let n: usize = 15;
    // let alpha_x: f64 = 2.5;
    // let sigma_a: f64 = 1.0;
    // let sigma_ex: f64 = 1.0;
    // rust infers the type based on context, so we could use the line below
    let (n, beta, alpha_y, alpha_x, sigma_a, sigma_ex, sigma_ey, intercept) = (10000, -0.5, 1.5, 2.5, 1.0, 1.0, 1.0, false);
    let params = (n, beta, alpha_y, alpha_x, sigma_a, sigma_ex, sigma_ey, &intercept);

    let ind_vars = match ind_var_generate(n, alpha_x, sigma_a, sigma_ex) {
        Ok(vars) => {
            // Access and print the generated values
            println!("v: {:?}", vars.v.slice(s![0..5, 0])); // just see the first 5 values
            println!("e_x: {:?}", vars.e_x.slice(s![0..5, 0]));
            println!("x: {:?}", vars.x.slice(s![0..5, 0]));
            println!("alpha_x: {}", vars.alpha_x);
            println!("sigma_a: {}", vars.sigma_a);
            println!("sigma_ex: {}", vars.sigma_ex);
            vars // return unwrapped output to be stored in ind_vars
        }
        Err(err_msg) => {
            eprintln!("Error generating independent variables: {}", err_msg);
            return;
        }
    }; // the match statement on its own does not make ind_vars available outside its scope

    match dep_var_generate(beta, alpha_y, sigma_ey, ind_vars) {
        Ok(dep_vars) => {
            // Access and print the generated values
            println!("y: {:?}", dep_vars.y.slice(s![0..5, 0]));
            println!("e_y: {:?}", dep_vars.e_y.slice(s![0..5, 0]));
            println!("alpha_y: {}", dep_vars.alpha_y);
            println!("beta: {}", dep_vars.beta);
            println!("sigma_ey: {}", dep_vars.sigma_ey);
            // println!("ind_vars: {:?}", dep_vars.ind_vars);
            // not printing ind_vars since that was checked already
        }
        Err(err_msg) => {
            eprintln!("Error generating dependent variables: {}", err_msg);
        }
    };

    // run the main regression
    let (regression_result, generated_data) = match run_yxv_regression(params) {
        Ok((regression_result, generated_data)) => {
            println!("Regression coefficients: {:?}", regression_result.params());
            // print the first row of y_data and x_data
            println!("First row of y: {:?}", generated_data.y.row(0));
            println!("First row of x: {:?}", generated_data.x.row(0));
            println!("First row of v: {:?}", generated_data.v.row(0));
            println!("First row of e_y: {:?}", generated_data.e_y.row(0));
            // print the other values
            println!("sigma_ex: {}", generated_data.sigma_ex);
            println!("sigma_a: {}", generated_data.sigma_a);
            println!("alpha_x: {}", generated_data.alpha_x);
            println!("alpha_y: {}", generated_data.alpha_y);
            (regression_result, generated_data) // return unwrapped results
        }
        Err(err_msg) => {
            eprintln!("Error with running yx regression: {}", err_msg);
        }
    };

    // run the other regressions
    match run_other_regressions(&generated_data, &intercept) {
        Ok((yx, vex)) => {
            println!("Regression Coefficients of y on x: {:?}", yx.params());
            println!("Regression Coefficients of composite error term on x: {:?}", vex.params()); 
        }
        Err(err_msg) => {
            eprintln!("Error with running the other regressions}", err_msg);
        }
    }
}
