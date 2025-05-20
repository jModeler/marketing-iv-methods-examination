use ndarray::Array2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Distribution;

/// Generates a random vector of length `n` using a specified 1D distribution.
///
/// This function uses the `ndarray_rand` crate to generate a 1-dimensional random vector
/// of length `n` from any distribution that implements the `Distribution<f64>` trait.
///
/// # Parameters
/// 
/// - `n`: The number of elements in the resulting random vector.
/// - `dist`: The distribution used to generate the random values. This must implement the
///   `Distribution<f64>` trait, which allows for various random distributions (e.g., uniform,
///   normal) from the `ndarray_rand` crate.
///
/// # Returns
/// 
/// - A `Array2<f64>` of shape `(n, 1)` containing the generated random values, where each value
///   is drawn from the provided distribution `dist`.
///
/// # Example
/// 
/// ```rust
/// use ndarray::Array2;
/// use ndarray_rand::rand_distr::Uniform;
/// use ndarray_rand::RandomExt;
/// 
/// let n = 10;
/// let dist = Uniform::new(0.0, 1.0);
/// let random_vector = random_vector_generate(n, dist);
/// println!("{:?}", random_vector);
/// ```
pub fn random_vector_generate<D>(n: usize, dist: D) -> Array2<f64>
where
    D: Distribution<f64> + 'static,
{
    Array2::random((n,1), dist)
}
