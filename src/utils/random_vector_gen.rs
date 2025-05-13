use ndarray::{Array, Array2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Normal;

pub fn random_vector_generate(n:usize) -> Array2<f64>{

    let random_vector = Array::random((n, 1), Normal::new(0., 9.).unwrap());

    return random_vector;
}
