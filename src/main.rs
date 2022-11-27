// use std::io;
use rand::Rng;

fn main() {
    let age = rand::thread_rng().gen_range(1..101);
    if(age >= 1) && (age <= 18) {
        println!("Important birthday.")
    }else if (age == 20) || (age == 50) {
        println!("Also important.")
    }
    else {
         println!("Unimportant")
    }
}
