//Conditional statements
pub fn run() {
    let age = 18;

    if age >= 21 {
        println!("Bartender: What do you want to drink?")
    } else {
        println!("Bartender: Sorry you have to leave")
    }

    //If expression
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age > 21 : {}", is_of_age);
}