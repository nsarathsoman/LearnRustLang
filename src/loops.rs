pub fn run() {
    let mut count = 0;

    //Infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    //while loop
    count = 1;
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count%3 == 0 {
            println!("Fizz");
        } else if count%5 == 0{            
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    for count in 0..100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count%3 == 0 {
            println!("Fizz");
        } else if count%5 == 0{            
            println!("Buzz");
        } else {
            println!("{}", count);
        }
    }
}