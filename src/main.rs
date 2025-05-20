mod utils;
mod simple_example;

use std::env;
use utils::plot_bias::plot_bias_vs_alpha_y;
use simple_example::run_regressions::{run_yxv_regression, run_other_regressions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- [bias | bias-size | main-regression | biased-regression]");
        return Ok(());
    }

    let command = &args[1];
    let beta = -0.5;
    let sigma_a = 1.0;
    let sigma_ex = 1.0;
    let sigma_ey = 1.0;
    let intercept = false;
    let alpha_y = 4.5;
    let alpha_x = 2.5;
    let n = 10000;
    let params = (n, beta, alpha_y, alpha_x, sigma_a, sigma_ex, sigma_ey, intercept);

    match command.as_str() {
        "bias" => {
            let (_, generated_data) = run_yxv_regression(params)?;
            let (_, _, bias) = run_other_regressions(&generated_data, intercept)?;
            println!("Bias in x coefficient: {}", bias);
        }

        "bias-size" => {
            let mut alpha_y_values = vec![];
            let mut bias_values = vec![];

            for alpha_y in (0..=20).map(|i| i as f64 * 0.1 + 1.0) {
                let params = (n, beta, alpha_y, alpha_x, sigma_a, sigma_ex, sigma_ey, intercept);

                match run_yxv_regression(params) {
                    Ok((_, generated_data)) => {
                        match run_other_regressions(&generated_data, intercept) {
                            Ok((_, _, bias)) => {
                                alpha_y_values.push(alpha_y);
                                bias_values.push(bias);
                            }
                            Err(e) => eprintln!("Error (other regressions) at alpha_y={}: {}", alpha_y, e),
                        }
                    }
                    Err(e) => eprintln!("Error (yxv regression) at alpha_y={}: {}", alpha_y, e),
                }
            }

            plot_bias_vs_alpha_y(&alpha_y_values, &bias_values, "bias_vs_alpha_y.png", "Bias vs alpha_y", "alpha_y", "Bias")?;
        }

        "main-regression" => {
            let (model, _) = run_yxv_regression(params)?;
            println!("Main regression (y ~ x + v) coefficients:");
            println!("{:?}", model.params());
        }

        "biased-regression" => {
            let (_, generated_data) = run_yxv_regression(params)?;
            let (biased_model, _, _) = run_other_regressions(&generated_data, intercept)?;
            println!("Biased regression (y ~ x) coefficients:");
            println!("{:?}", biased_model.params());
        }

        _ => {
            eprintln!("Unrecognized command: {}", command);
            eprintln!("Usage: cargo run -- [bias | bias-size | main-regression | biased-regression]");
        }
    }

    Ok(())
}
