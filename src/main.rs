// use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let age = rand::thread_rng().gen_range(1..101);
    let voteing_age = 18;
    match age.cmp(& voteing_age) {
        Ordering::Less => println!("You can't vote."),
        Ordering::Greater => println!("You can vote."),
        Ordering::Equal => println!("Congratulation."),
    }
}
