mod utils;

use utils::simple_example::{ind_var_generate, dep_var_generate};

fn main() {
    // let n: usize = 15;
    // let alpha_x: f64 = 2.5;
    // let sigma_a: f64 = 1.0;
    // let sigma_ex: f64 = 1.0;
    // rust infers the type based on context, so we could use the line below
    let (n, beta, alpha_y, alpha_x, sigma_a, sigma_ex, sigma_ey) = (15, 0.5, 1.5, 2.5, 1.0, 1.0, 1.0);


    let ind_vars = match ind_var_generate(n, alpha_x, sigma_a, sigma_ex) {
        Ok(vars) => {
            // Access and print the generated values
            println!("v: {:?}", vars.v);
            println!("e_x: {:?}", vars.e_x);
            println!("x: {:?}", vars.x);
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
            println!("y: {:?}", dep_vars.y);
            println!("e_y: {:?}", dep_vars.e_y);
            println!("alpha_y: {}", dep_vars.alpha_y);
            println!("beta: {}", dep_vars.beta);
            println!("sigma_ey: {}", dep_vars.sigma_ey);
            println!("ind_vars: {:?}", dep_vars.ind_vars);
        }
        Err(err_msg) => {
            eprintln!("Error generating independent variables: {}", err_msg);
        }
    }
}
