use marketing_iv_methods::utils::simple_example::{ind_var_generate, IndVars};

fn valid_input() -> (usize, f64, f64, f64) {
    (10, 2.5, 1.0, 0.5)
}

fn generate_valid_data() -> IndVars {
    let (n, alpha_x, sigma_a, sigma_ex) = valid_input();
    ind_var_generate(n, alpha_x, sigma_a, sigma_ex).unwrap()
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
fn test_x_shape_matches_v() {
    let data = generate_valid_data();
    assert_eq!(data.x.shape(), data.v.shape());
}

