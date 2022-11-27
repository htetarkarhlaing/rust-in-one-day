fn main() {
    let mut str_one = String::new();
    str_one.push('A');
    str_one.push_str(" Word");

    for word in str_one.split_whitespace() {
        println!("{}", word);
    }

    let str_two = str_one.replace("A", "Another");
    println!("{}", str_two);
}
