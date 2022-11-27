// use std::io;
// use rand::Rng;

fn main() {
    const ONE_MILE: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    let age = "21";
    let age: u32 = age.parse().expect("Age was not assigned a number.");
    // age = age + 1;
    println!("I'm {} years old and I went {} miles.", age, ONE_MILE);
}
