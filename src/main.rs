mod utils;

use utils::simple_example::ind_var_generate;

fn main() {
    // let n: usize = 15;
    // let alpha_x: f64 = 2.5;
    // let sigma_a: f64 = 1.0;
    // let sigma_ex: f64 = 1.0;
    // rust infers the type based on context, so we could use the line below
    let (n, alpha_x, sigma_a, sigma_ex) = (15, 2.5, 1.0, 1.0);


    match ind_var_generate(n, alpha_x, sigma_a, sigma_ex) {
        Ok(ind_vars) => {
            // Access and print the generated values
            println!("v: {:?}", ind_vars.v);
            println!("e_x: {:?}", ind_vars.e_x);
            println!("x: {:?}", ind_vars.x);
            println!("alpha_x: {}", ind_vars.alpha_x);
            println!("sigma_a: {}", ind_vars.sigma_a);
            println!("sigma_ex: {}", ind_vars.sigma_ex);
        }
        Err(err_msg) => {
            eprintln!("Error generating independent variables: {}", err_msg);
        }
    }
}
