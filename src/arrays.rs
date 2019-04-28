//Arrays - Fixed size list of homegenous values

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //Assigning val
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single value
    println!("Single val: {}", numbers[0]);

    //Array length
    println!("Length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}