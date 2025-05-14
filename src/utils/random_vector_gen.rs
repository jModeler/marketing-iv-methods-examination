use ndarray::Array2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Distribution;

// Generic function to generate a random vector from any 1D distribution available in ndarray_rand
pub fn random_vector_generate<D>(n: usize, dist: D) -> Array2<f64>
where
    D: Distribution<f64> + 'static,
{
    Array2::random((n,1), dist)
}

// function to create 