/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128 (number of bits they take in memory)
Floats: f32, f64
Boolean: (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time
// However, the compiler can usually infer what type we want to use
// based on the value and how we use it

pub fn run() {
    let x = 1; // [supersudh] by default this is gonna be an i32

    let y = 3.47; // By default this is gonna be a {float 64} f64

    let z: i64 = 208934702937480234;

    println!("Max i32: {}", std::i32::MAX); // Find max size // [supersudh] std is standard library
    println!("Max i64: {}", std::i64::MAX); // Find max size

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater = 10 > 5;

    let a1 = 'a'; // Using single quote for char
                  //  let face = '\u{1F600}'; TODO: supersudh [Emojis are not supported 😔 ]

    //  let errorChar ='akjsd' // do not use more than one char in single quotes in RUST

    println!("{:?}", (x, y, z, is_active, is_greater, a1)) // We wish we fixed the error ^^ 😔
}
