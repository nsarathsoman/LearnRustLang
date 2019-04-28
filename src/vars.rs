//Variable hold primitive data or reference to data
//Variables are immutable by default
//Rust is a block-scoped language
pub fn run() {
    let name = "Sarath";
    let mut age = 28;
    println!("Helo, {} , {}", name, age);

    age = 29;
    println!("Helo, {} , {}", name, age);

    //Const needs explicit type annotation for variables;
    const ID: i32 = 001;
    println!("Constatn {}", ID);

    let (my_name, my_age) = ("Sarath", 29);
    println!("{} is {}", my_name, my_age);
}