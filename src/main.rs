mod utils;

use utils::random_vector_gen::random_vector_generate;

fn main() {
    let random_vector = random_vector_generate(10);
    println!("Randomly generated 10x1 vector: {:?}", random_vector);
}