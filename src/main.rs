fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    return sum;
}

fn get_two(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}
fn main() {
    println!("{:?}", get_two(4));
    let (val1, val2) = get_two(5);
    println!("Here we go two values, {} & {}", val1, val2);

    let num_list: Vec<i32> = vec![1, 2, 3, 4];
    println!("Total : {}", sum_list(&num_list))
}
