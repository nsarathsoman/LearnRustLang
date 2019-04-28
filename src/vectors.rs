//Arrays - resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //Assigning val
    numbers[2] = 20;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Pop off last values
    numbers.pop();

    println!("{:?}", numbers);

    //Get single value
    println!("Single val: {}", numbers[0]);

    //Vector length
    println!("Length: {}", numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter() {
        println!("Number {}", x);
    }

    //Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers: {:?}", numbers);

}