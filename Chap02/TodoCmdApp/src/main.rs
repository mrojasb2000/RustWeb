use clap::{App, Arg};
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
    let app = App::new("booking")
        .version("1.0.0")
        .author("Maxwell Flitton")
        .about("Books in a user");

    let first_name = Arg::new("first name of user")
        .long("f")
        .takes_value(true)
        .required(true)
        .help("first name of user");

    let last_name = Arg::new("last name of user")
        .long("l")
        .takes_value(true)
        .required(true)
        .help("last name of user");

    let age = Arg::new("age")
        .long("a")
        .takes_value(true)
        .required(true)
        .help("age of the user");

    let app = app.arg(first_name).arg(last_name).arg(age);
    let matches = app.get_matches();

    let name = matches
        .value_of("first name of user")
        .expect("First name is required");
    let surname = matches
        .value_of("last name of user")
        .expect("Last name is required");
    let age: i8 = matches
        .value_of("age")
        .expect("Age is required")
        .parse()
        .unwrap();

    println!("First name: {}", name);
    println!("Last name: {}", surname);
    println!("Age: {}", age);

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
