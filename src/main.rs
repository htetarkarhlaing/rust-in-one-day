fn main() {
    let vec_one: Vec<i32> = Vec::new();
    let mut vec_two = vec![1, 2, 3, 4, 5];
    vec_two.push(6);
    println!("1st : {}", vec_two[0]);
    let second: &i32 = &vec_two[1];
    match vec_two.get(1) {
        Some(second) => println!("2nd is : {}", second),
        None => println!("No 2nd value"),
    };

    for i in &mut vec_two {
        *i *= 2;
    };

    for i in &vec_two {
        println!("{}", i);
    };

    println!("Vec length, {}", vec_two.len());
    println!("Pop: {:?}", vec_two.pop());
}
