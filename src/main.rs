mod utils;

use utils::random_vector_gen::random_vector_generate;
use ndarray_rand::rand_distr::Normal;

fn main() {
    let dist = Normal::new(0.0, 9.0).unwrap();
    let random_vector = random_vector_generate(10, dist);
    println!("Randomly generated 10x1 vector: {:?}", random_vector);
}