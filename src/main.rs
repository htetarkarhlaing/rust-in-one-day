// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    let my_tuple: (u8, String, f64) = (47, "John Snow".to_string(), 50_000.005);
    println!("Name: {}", my_tuple.1);

    let (v1, v2, v3) = my_tuple;
    println!("Age: {} and loan: {} is {}", v1, v3, v2);
}
