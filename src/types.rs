/*
Primitive types
Integers : u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (num of bits they take)
Float: f32, f64
Boolean: bool
Characters: char
Tuples 
Arrays
*/
pub fn run() {
    //default is i32
    let x = 1;

    //default is f64
    let y = 2.5;

    //Explicit type
    let z: i64 = 121231312312313123;

    //Find MAX
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);

    //Boolean
    let is_active: bool = true;

    //bool from expression
    let is_gt = 10 < 5;

    //char 
    let a1: char = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_gt, a1, face));

}