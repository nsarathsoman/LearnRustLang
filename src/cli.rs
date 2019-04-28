use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    if args[1] == "hello" {
        println!("Hello");
    }
}