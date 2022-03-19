/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn types() {
    // Default is i32
    let x = 1;

    let y = 2;
    // Default is f64

    // explicit type cast
    let z: i64 = 4657593479879;
    // Max size
    println!("Max i32: {} ", std::i32::MAX);
    println!("Max i64: {} ", std::i64::MAX);

    //  Boolean
    let data_s = true;

    // let the greater expression
    let is_greater: bool = 10 > 55;

    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (face,x, y, z, data_s, is_greater, a1));
}
