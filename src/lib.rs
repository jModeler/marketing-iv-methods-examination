use rand::Rng;

pub fn random_number_print() {

    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("Randomly Generated Number: {random_number}");
}