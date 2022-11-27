fn say_hello() {
    println!("Hello World");
}

fn get_sum(x: u32, y: u32) {
    println!("{} + {} = {}", x, y, x + y)
}

fn get_sum_value(x: i32, y: i32) -> i32 {
    x + y
}
fn main() {
    say_hello();
    get_sum(5, 6);
    println!("{}", get_sum_value(5, 6));
}
