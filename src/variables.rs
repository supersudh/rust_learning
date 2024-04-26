// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language [supersudh] This would remind us of const, let, var confusions in JS :-P

pub fn run() {
    let name = "San Redux";
    let mut age = 30;
    println!("My name is {} and I am {}", name, age); // [supersudh]sequence of code line is important here
    age = 100; // Reassign/mutate only for reuse in subsequent lines
    println!("My name is {} and I am {}", name, age); // Note to self: Sequence of code is important in Rust
}
