//Struct used to create custom data types

//Traditional Struct
struct Color {
    red: u8,
    blue: u8,
    green: u8,
}

//Tuple Struct
struct TColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name : first_name.to_string(),
            last_name : last_name.to_string()
        }
    }

    //Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string();
    }

    //to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
pub fn run() {
    let mut c = Color {
        red: 255,
        blue: 0,
        green: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.blue, c.green);


    let mut c1 = TColor(250, 100, 10);
    c1.0 = 150;

    println!("Color: {} {} {}", c1.0, c1.1, c1.2);

    let mut p = Person::new("Sarath", "Soman");
    println!("Person: {} {}", p.first_name, p.last_name);
    p.set_last_name("S");
    println!("Person Full Name: {}", p.full_name());
    println!("Person Tuple: {:?}", p.to_tuple());
}
