use marketing_iv_methods::utils::random_vector_gen::random_vector_generate;
use ndarray_rand::rand_distr::{Normal, Uniform};
use ndarray_stats::SummaryStatisticsExt;
use ndarray::Axis;
use approx::abs_diff_eq;

#[test]
fn test_vector_size_normal() {
    let dist = Normal::new(0.0, 1.0).unwrap();
    let vec = random_vector_generate(5, dist);
    assert_eq!(vec.shape(), &[5, 1])
}

#[test]
fn test_vector_size_uniform() {
    let dist = Uniform::new(0.0, 1.0);
    let vec = random_vector_generate(5, dist);
    assert_eq!(vec.shape(), &[5, 1])
}

#[test]
fn test_vector_stats_normal() {
    let dist = Normal::new(0.0, 9.0).unwrap();
    let vec = random_vector_generate(10000, dist);
    let flattened = vec.view().into_shape(data.len()).unwrap();

    // sample mean 
    let mean = flattened.mean().unwrap();

    // sample standard deviation
    let std = flattened.std(Axis(0), 1.0);

    // compare to population mean and standard deviation
    assert!(abs_diff_eq!(mean, 0.0, epsilon = 1e-2));
    assert!(abs_diff_eq!(std, 1.0, epislon = 1e-2));
}