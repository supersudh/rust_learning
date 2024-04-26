pub fn run() {
    // public function
    println!("Hello from the print.rs file");

    // println!("Number: {}", 1); // Correct

    // WRONG println(1) -> leads to error

    println!("{} is from {}", "Sudharsan", "UK"); // String interpolation

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to play{2}",
        "Sudharsan", "UK", "chess"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Sudharsan",
        activity = "chess"
    );

    // Placeholder traits
    // for Binary that trait is :b
    // for hex trait is :x
    // for octal trait is :o
    println!("Binary {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}
