use marketing_iv_methods::utils::random_vector_gen::random_vector_generate;
use ndarray_rand::rand_distr::{Normal, Uniform};

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
    let vec = random_vector_generate(1000000, dist);
    let binding = vec.view();
    let flattened = binding.to_shape(vec.len()).unwrap();

    // sample mean 
    let mean = flattened.mean().unwrap();

    // sample standard deviation
    let std = flattened.std(1.0);

    // compare to population mean and standard deviation
    assert!((mean-0.0).abs() < 1e-2, "Expected values to be close, but got mean = {}", mean);
    assert!((std - 9.0).abs() < 1e-2, "Expected values to be close, but got standard deviation = {}", std);
}