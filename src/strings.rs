//Two types of String
//Primitive str - Immutable fixed length string somewhere in the memory
//String - Growable, heap allocated, use when you need to own the data - modify etc

pub fn run() {
    let str1 = "Hello, ";

    let mut message = String::from(str1);
    message.push_str("World");

    println!("{}", message);
    println!("Length: {}", message.len());

    //Capacity in bytes
    println!("Capacity: {}", message.capacity());

    //Is Empty
    println!("Is Empty: {}", message.is_empty());

    //String with capacity
    let mut a1 = String::with_capacity(10);
    a1.push('1');
    a1.push('2');
    assert_eq!(2, a1.len());
    assert_eq!(10, a1.capacity());
}