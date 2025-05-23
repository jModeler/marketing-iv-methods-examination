use crate::utils::random_vector_gen::random_vector_generate; 
use ndarray::Array2;
use ndarray_rand::rand_distr::Normal;

/// A structure to store the independent variables generated by `ind_var_generate`.
///
/// This structure holds the generated data for the independent variables, including:
/// - `v`: The generated random variable `v`.
/// - `e_x`: The generated random error term `e_x`.
/// - `x`: The calculated independent variable `x` (a linear combination of `v` and `e_x`).
/// - `alpha_x`: The coefficient of `v`.
/// - `sigma_a`: The standard deviation of `v`.
/// - `sigma_ex`: The standard deviation of the error term `e_x`.
///
/// # Example
/// 
/// ```rust
/// use ndarray::Array2;
/// use marketing_iv_methods::simple_example::generate_vector_data::IndVars;
///
/// let ind_vars = IndVars {
///     v: Array2::zeros((5, 1)),
///     e_x: Array2::zeros((5, 1)),
///     x: Array2::zeros((5, 1)),
///     alpha_x: 2.5,
///     sigma_a: 1.0,
///     sigma_ex: 1.0,
/// };
/// ```
#[derive(Debug)]  // for testing purposes later
pub struct IndVars {
    pub v: Array2<f64>,
    pub e_x: Array2<f64>,
    pub x: Array2<f64>,
    pub alpha_x: f64,
    pub sigma_a: f64,
    pub sigma_ex: f64,
}

/// A structure to store the dependent variables generated by `dep_var_generate`.
///
/// This structure holds the generated data for the dependent variables, including:
/// - `y`: The generated dependent variable `y`.
/// - `e_y`: The generated error term `e_y`.
/// - `alpha_y`: The coefficient of `v` in the regression equation.
/// - `beta`: The coefficient of `x` in the regression equation.
/// - `sigma_ey`: The standard deviation of the error term `e_y`.
/// - `ind_vars`: The independent variables that were used to generate `y`.
///
/// # Example
/// 
/// ```rust
/// use ndarray::Array2;
/// use marketing_iv_methods::simple_example::generate_vector_data::IndVars;
/// use marketing_iv_methods::simple_example::generate_vector_data::DepVars;
///
/// let dep_vars = DepVars {
///     y: Array2::zeros((5, 1)),
///     e_y: Array2::zeros((5, 1)),
///     alpha_y: 1.5,
///     beta: -0.5,
///     sigma_ey: 1.0,
///     ind_vars: IndVars {
///         v: Array2::zeros((5, 1)),
///         e_x: Array2::zeros((5, 1)),
///         x: Array2::zeros((5, 1)),
///         alpha_x: 2.5,
///         sigma_a: 1.0,
///         sigma_ex: 1.0,
///     },
/// };
/// ```
#[derive(Debug)]
pub struct DepVars {
    pub y: Array2<f64>,
    pub e_y: Array2<f64>,
    pub alpha_y: f64,
    pub beta: f64,
    pub sigma_ey: f64,
    pub ind_vars: IndVars,
}

/// Generates independent variables for a regression model.
///
/// This function generates a set of independent variables, including:
/// - A random error term `v` with standard deviation `sigma_a`.
/// - A random error term `e_x` with standard deviation `sigma_ex`.
/// - The independent variable `x`, which is calculated as `x = alpha_x * v + e_x`.
///
/// The function returns a `Result` that contains an `IndVars` struct with the generated data.
///
/// # Parameters
/// 
/// - `n`: The number of observations (i.e., the number of rows in the generated arrays).
/// - `alpha_x`: The coefficient for the variable `v`.
/// - `sigma_a`: The standard deviation for the error term `v`.
/// - `sigma_ex`: The standard deviation for the error term `e_x`.
///
/// # Returns
/// 
/// Returns a `Result<IndVars, String>`, where `Ok` contains the generated independent variables, 
/// and `Err` contains an error message if the inputs are invalid (e.g., non-positive standard deviations).
///
/// # Example
/// 
/// ```rust
/// use ndarray::Array2;
/// use marketing_iv_methods::simple_example::generate_vector_data::IndVars;
/// use marketing_iv_methods::simple_example::generate_vector_data::ind_var_generate;
///
/// let result = ind_var_generate(100, 2.5, 1.0, 1.0);
/// match result {
///     Ok(ind_vars) => {
///         println!("{:?}", ind_vars);
///     }
///     Err(err) => {
///         println!("Error: {}", err);
///     }
/// }
/// ```
pub fn ind_var_generate(n: usize, alpha_x: f64, sigma_a: f64, sigma_ex: f64) -> Result<IndVars, String> {
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

/// Generates dependent variables for a regression model using the independent variables.
///
/// This function generates the dependent variable `y` based on the formula:
/// 
/// `y = beta * x + alpha_y * v + e_y`, where:
/// - `x` and `v` are the independent variables from the `IndVars` structure.
/// - `beta` is the coefficient for the independent variable `x`.
/// - `alpha_y` is the coefficient for the independent variable `v`.
/// - `e_y` is the error term generated with standard deviation `sigma_ey`.
///
/// The function returns a `Result` containing a `DepVars` struct with the generated data.
///
/// # Parameters
/// 
/// - `beta`: The coefficient for the independent variable `x`.
/// - `alpha_y`: The coefficient for the independent variable `v`.
/// - `sigma_ey`: The standard deviation for the error term `e_y`.
/// - `ind_vars`: The independent variables, returned by `ind_var_generate`.
///
/// # Returns
/// 
/// Returns a `Result<DepVars, String>`, where `Ok` contains the generated dependent variables, 
/// and `Err` contains an error message if the inputs are invalid (e.g., non-positive standard deviation).
///
/// # Example
/// 
/// ```rust
/// use ndarray::Array2;
/// use marketing_iv_methods::simple_example::generate_vector_data::IndVars;
/// use marketing_iv_methods::simple_example::generate_vector_data::dep_var_generate;
///
/// let ind_vars = IndVars {
///     v: Array2::zeros((5, 1)),
///     e_x: Array2::zeros((5, 1)),
///     x: Array2::zeros((5, 1)),
///     alpha_x: 2.5,
///     sigma_a: 1.0,
///     sigma_ex: 1.0,
/// };
/// let result = dep_var_generate(-0.5, 1.5, 1.0, ind_vars);
/// match result {
///     Ok(dep_vars) => {
///         println!("{:?}", dep_vars);
///     }
///     Err(err) => {
///         println!("Error: {}", err);
///     }
/// }
/// ```
pub fn dep_var_generate(beta: f64, alpha_y: f64, sigma_ey: f64, ind_vars: IndVars) -> Result<DepVars, String> {
    // check that sigma_ey is positive
    if sigma_ey <= 0.0 {
        return Err("sigma_ey must be positive".into());
    }

    // get the number of rows from x (or v, since both should be the same size)
    let n = ind_vars.x.nrows();

    // generate the distribution of the error term
    let dist_e = Normal::new(0.0, sigma_ey).unwrap();
    
    // generate the error term
    let e_y = random_vector_generate(n, dist_e);

    // generate y, the dependent variable
    let y = beta * &ind_vars.x + alpha_y * &ind_vars.v + &e_y;
    // return a data struct holding the values generated
     Ok(DepVars {
        y,
        e_y,
        alpha_y,
        beta,
        sigma_ey,
        ind_vars,
    }) 
}