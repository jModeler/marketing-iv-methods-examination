use marketing_iv_methods::utils::simple_example::{ind_var_generate, dep_var_generate, IndVars, DepVars};

fn valid_input() -> (usize, f64, f64, f64) {
    (10, 2.5, 1.0, 0.5)
}

fn generate_valid_data() -> IndVars {
    let (n, alpha_x, sigma_a, sigma_ex) = valid_input();
    ind_var_generate(n, alpha_x, sigma_a, sigma_ex).unwrap()
}

fn valid_input_y() -> (f64, f64, f64, IndVars) {
    let (n, alpha_x, sigma_a, sigma_ex) = valid_input();
    let ind_vars = ind_var_generate(n, alpha_x, sigma_a, sigma_ex).unwrap();
    (0.5, 1.5, 1.0, ind_vars)
}

fn generate_valid_data_y() -> DepVars {
    let (beta, alpha_y, sigma_ey, ind_vars) = valid_input_y();
    dep_var_generate(beta, alpha_y, sigma_ey, ind_vars).unwrap()
}

// Fixture: returns input with negative sigma_a
fn input_with_negative_sigma_a() -> (usize, f64, f64, f64) {
    let (n, alpha_x, _, sigma_ex) = valid_input();
    (n, alpha_x, -1.0, sigma_ex)
}

// Fixture: returns input with negative sigma_ex
fn input_with_negative_sigma_ex() -> (usize, f64, f64, f64) {
    let (n, alpha_x, sigma_a, _) = valid_input();
    (n, alpha_x, sigma_a, -0.5)
}

//Fixture: retuns input with negative sigma_ey
fn input_with_negative_sigma_ey() -> (f64, f64, f64, IndVars) {
    let (beta, alpha_y, _, ind_vars) = valid_input_y();
    (beta, alpha_y, -1.0, ind_vars)
}

#[test]
fn test_errors_on_negative_sigma_a() {
    let (n, alpha_x, sigma_a, sigma_ex) = input_with_negative_sigma_a();
    let result = ind_var_generate(n, alpha_x, sigma_a, sigma_ex);
    assert!(result.is_err(), "Expected error for negative sigma_a, but got Ok");
    assert_eq!(result.unwrap_err(), "sigma_a must be positive");
}

#[test]
fn test_errors_on_negative_sigma_ex() {
    let (n, alpha_x, sigma_a, sigma_ex) = input_with_negative_sigma_ex();
    let result = ind_var_generate(n, alpha_x, sigma_a, sigma_ex);
    assert!(result.is_err(), "Expected error for negative sigma_ex, but got Ok");
    assert_eq!(result.unwrap_err(), "sigma_ex must be positive");
}

#[test]
fn test_errors_on_negative_sigma_ey() {
    let (beta, alpha_y, sigma_ey, ind_vars) = input_with_negative_sigma_ey();
    let result = dep_var_generate(beta, alpha_y, sigma_ey, ind_vars);
    assert!(result.is_err(), "Expected error for negative sigma_ex, but got Ok");
    assert_eq!(result.unwrap_err(), "sigma_ey must be positive");
}

#[test]
fn test_x_shape_matches_v() {
    let data = generate_valid_data();
    assert_eq!(data.x.shape(), data.v.shape());
}

#[test]
fn test_y_shape_matches_v() {
    let data = generate_valid_data_y();
    assert_eq!(data.y.shape(), data.ind_vars.v.shape());
}

#[test]
fn test_x_value() {
    let data = generate_valid_data();
    let x_expected = &data.v * data.alpha_x + &data.e_x;
    assert_eq!(data.x, x_expected);
}

#[test]
fn test_y_value() {
    let data = generate_valid_data_y();
    let y_expected = &data.ind_vars.x * data.beta + &data.ind_vars.v * data.alpha_y + &data.e_y;
    assert_eq!(data.y, y_expected);
}