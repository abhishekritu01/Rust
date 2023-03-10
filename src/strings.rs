// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // let hello = String::from("Hello");
    let mut hello = String::from("Hello");

    //get length
    println!("Length:{}", hello.len());

    //push char
    hello.push('W');

    //push string
    hello.push_str("orld");

    //capacity in bytes
    println!("Capacity:{}", hello.capacity());

    //isEmpty
    println!("Is Empty:{}", hello.is_empty());

    //contains
    println!("Contains 'World' {}", hello.contains("World"));

    //replace
    println!("Replace: {}", hello.replace("World", " there"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //create string with capacity
    let mut s =String::with_capacity(10);
    s.push('a');
    s.push('b');
    // println!("{}",s);

    // Assertation Test
    assert_eq!(2,s.len());
    assert_eq!(11,s.capacity());
    println!("{}",s);

}
 