use marketing_iv_methods::utils::random_vector_gen::random_vector_generate;
use ndarray_rand::rand_distr::{Normal, Uniform};

#[test]
fn test_vector_size() {
    let dist = Normal::new(0.0, 1.0).unwrap();
    let vec = random_vector_generate(5, dist);
    assert_eq!(vec.shape(), &[5, 1])
}