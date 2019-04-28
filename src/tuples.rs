//max 12 values
//ordered collection of values
pub fn run() {
    let person: (&str, &str, i8) = ("Sarath", "Paravur", 29);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}