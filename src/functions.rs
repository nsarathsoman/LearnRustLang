pub fn run() {
    greetings("hello", "Sarath");

    //bind func return to var
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);

    //closure
    let n3 = 10;
    let add_sum = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("10 + 20 + 10= {}", add_sum(10, 20));
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}