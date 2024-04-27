// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own
// string data

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get String length
    print!("Length: {}", hello.len());

    hello.push('S'); // hello.push('W') [NOTE TO SELF] this will yield error if mut prefix is missing in hello

    // Lets push a string now instead of a char
    hello.push_str("UDHARSAN");

    // Capacity in Bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));
    println!("Contains 'SUDHARSAN' {}", hello.contains("SUDHARSAN"));

    // Replace
    println!("Replace: {}", hello.replace("SUDHARSAN", "RAVIKUMAR")); // functional programming ?? 😕

    // Loop through string by whitespace so we can use a for loop
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('x');
    s.push('y');

    // Assertion testing
    assert_eq!(2, s.len());

    assert_eq!(3, s.len()); // This apparently throws false
    assert_eq!(10, s.capacity()); // This apparently doesn't throw

    println!("{}", hello)
}
