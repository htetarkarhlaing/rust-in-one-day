use std::string;

fn main() {
    let str_one = String::from("j k y u d f g k k d e s");
    let mut v1: Vec<char> = str_one.chars().collect();
    v1.sort();
    v1.dedup();
    for chr in v1 {
        println!("Char : {}", chr);
    }

    let str_two: &str = "Random string";
    let mut heap_str: String = str_two.to_string();
    println!("{}", heap_str);

    let byte_of_heap_str = heap_str.as_bytes();
    // println!("Bytes: {}", byte_of_heap_str);
    let str_three = &heap_str[0..6];
    println!("String lenght: {}", str_three.len());
    heap_str.clear();
    let str_four = String::from("Just some");
    let str_five = String::from(" words");
    let str_six = str_four + &str_five;

    for char in str_six.bytes() {
        println!("{}", char)
    }
}
