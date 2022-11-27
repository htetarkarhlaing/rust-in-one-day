// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    loop {
        if arr[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        if arr[loop_index] == 9 {
            break;
        }
        println!("Val : {}", arr[loop_index]);
        loop_index += 1;
    }

    // while loop
    while loop_index < arr.len() {
        println!("Arr : {}", arr[loop_index]);
        loop_index += 1;
    }
}
