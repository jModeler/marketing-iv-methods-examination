mod utils;
mod simple_example;

use std::env;
use utils::plotting::plot_bias_vs_alpha_y;
use simple_example::run_regressions::{run_yxv_regression, run_other_regressions, analyze_bias};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- [bias | bias-size]");
        return Ok(());
    }

    let command = &args[1];
    let beta = -0.5;
    let sigma_a = 1.0;
    let sigma_ex = 1.0;
    let sigma_ey = 1.0;
    let intercept = false;

    match command.as_str() {
        "bias" => {
            let alpha_y = 1.5;
            let alpha_x = 2.5;
            let n = 10000; // or any value you prefer
            let params = (n, beta, alpha_y, alpha_x, sigma_a, sigma_ex, sigma_ey, intercept);
            
            // Print out the bias based on the regression
            analyze_bias(params);
        }
        "bias-size" => {
            let mut alpha_y_values = vec![];
            let mut bias_values = vec![];

            // Loop over a range of alpha_y values
            for alpha_y in (0..=20).map(|i| i as f64 * 0.1 + 1.0) { // e.g., from 1.0 to 3.0
                let alpha_x = 2.5; // fixed alpha_x
                let n = 10000; // fixed sample size, or loop over different n if desired
                let params = (n, beta, alpha_y, alpha_x, sigma_a, sigma_ex, sigma_ey, intercept);

                match run_yxv_regression(params) {
                    Ok((regression_result, generated_data)) => {
                        match run_other_regressions(&generated_data, intercept) {
                            Ok((yx, _, _)) => {
                                // Extract the biased coefficient of x from the regression
                                let coef_x_biased = yx.params()[0];
                                let bias = coef_x_biased - beta; // Bias is the difference from true beta
                                alpha_y_values.push(alpha_y);
                                bias_values.push(bias);
                            }
                            Err(e) => eprintln!("Error (other regressions) at alpha_y={}: {}", alpha_y, e),
                        }
                    }
                    Err(e) => eprintln!("Error (yxv regression) at alpha_y={}: {}", alpha_y, e),
                }
            }

            // Plot bias values for different alpha_y values
            plot_bias_vs_alpha_y(&alpha_y_values, &bias_values, "bias_vs_alpha_y.png", "Bias vs alpha_y", "alpha_y", "Bias")?;
        }
        _ => {
            eprintln!("Unrecognized command: {}", command);
            eprintln!("Usage: cargo run -- [bias | bias-size]");
        }
    }

    Ok(())
}
