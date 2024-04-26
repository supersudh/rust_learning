// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language [supersudh] This would remind us of const, let, var confusions in JS :-P

pub fn run() {
    let name = "San Redux";
    let age = 30;

    // age = 100 // ERROR!

    println!("My name is {} and I am {}", name, age)
}
