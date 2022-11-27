fn main() {
    let int_u8: u8 = 10;
    let int2_u8: u8 = 30;

    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("{}", int3_u32);
}
