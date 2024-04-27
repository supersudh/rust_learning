// Tuples group together values of different types
// Max ?12? elements //[supersudh] todo

pub fn run() {
    let person: (&str, &str, i8) = ("Sudharsan", "England", 30);

    println!("{} is from {} and is {}", person.0, person.1, person.2)
}