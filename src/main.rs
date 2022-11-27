// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    let age = 8;
    match age {
        1..=18 => println!("Important birthday."),
        22 | 50 => println!("Importnat birthday."),
        65..=u32::MAX => println!("Important birthday"),
        _ => println!("Not important birthday."),
    };
}
