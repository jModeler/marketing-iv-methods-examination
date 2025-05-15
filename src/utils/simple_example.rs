use super::random_vector_gen::random_vector_generate; 
// above statement is equivalent to use marketing_iv_methods::utils::random_vector_gen::random_vector_generate, 
// used this since simple_example.rs and random_vector_gen.rs are in the same folder;
use ndarray::Array2;
use ndarray_rand::rand_distr::Normal;

// define a structure to store the data returned by ind_var_generate
#[derive(Debug)]  // for testing purposes later
pub struct IndVars {
    pub v: Array2<f64>,
    pub e_x: Array2<f64>,
    pub x: Array2<f64>,
    pub alpha_x: f64,
    pub sigma_a: f64,
    pub sigma_ex: f64,
}

pub fn ind_var_generate(n: usize, alpha_x: f64, sigma_a: f64, sigma_ex: f64) -> Result<IndVars, String> {
    /*
    n: number of observations, or the number of rows in the array being generated
    alpha_x: coefficient of v
    sigma_a: the standard deviation of v, has to be positive
    sigma_ex: the standard deviation of the error term, e_x, has to be positive
    */

    // check that sigma values are positive
    if sigma_a <= 0.0 {
        return Err("sigma_a must be positive".into());
    }
    if sigma_ex <= 0.0 {
        return Err("sigma_ex must be positive".into());
    }

    // generate the error term
    let dist_v = Normal::new(0.0, sigma_a).unwrap(); 
    let dist_e = Normal::new(0.0, sigma_ex).unwrap();

    let v = random_vector_generate(n, dist_v);
    let e_x = random_vector_generate(n, dist_e);

    // generate the other independent variable, x
    let x = alpha_x * &v + &e_x; //& helps me borrow v and e_x immutably so I can use them later

    // return a data struct holding the values generated
     Ok(IndVars {
        v,
        e_x,
        x,
        alpha_x,
        sigma_a,
        sigma_ex,
    }) 
}