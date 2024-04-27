// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own
// string data

pub fn run() {
    let hello = String::from("Hello");

    // Get String length
    print!("Length: {}", hello.len());

    println!("{}", hello)

}