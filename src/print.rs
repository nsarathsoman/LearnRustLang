pub fn run() {
    //Hello world
    println!("Hello, World");

    //Formatting
    println!("{}, {} is here", "Sarath", "Binsa");

    //positional arguments
    println!("{0} is from {1}. {0} codes in {2}", "Sarath", "Paravur", "Java");

    //Names arguments
    println!("{name} like {activity}", name = "Sarath", activity = "Programming");

    //Plave holder traits
    println!("Binary {:b}, Hex {:x} Octal {:o}", 10, 10, 10);

    //Place holder for debug traits
    println!("{:?}", (12, true, "Hell"));
}