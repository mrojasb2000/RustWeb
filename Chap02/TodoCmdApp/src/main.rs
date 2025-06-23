use rand::prelude::*;
use std::env;

/// This function generates a float number using a number
/// generator passed into the function.
///
/// # Arguments
/// * generator (&mut ThreadRng): the random number
/// genarator to generate the random number
///
/// # Returns
/// (f64): random number between 0 -> 10
fn generate_float(generator: &mut ThreadRng) -> f64 {
    let placeholder: f64 = generator.random();
    return placeholder * 10.0;
}

fn main() {
    let mut rng: ThreadRng = rand::rng();
    let random_number = generate_float(&mut rng);
    println!("{}", random_number);

    let args: Vec<String> = env::args().collect();
    let path: &str = &args[0];
    println!("{:?}", args);

    if path.contains("/debug/") {
        println!("Debug is running");
    } else if path.contains("/release/") {
        println!("release is running");
    } else {
        panic!("The setting is neither debug nor release");
    }
}
