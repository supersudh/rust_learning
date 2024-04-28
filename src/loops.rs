pub fn run() {
    let mut count = 0;

    // ∞ loop

    // loop {
    //     count += 1;
    //     println!("Number {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While loop (buzzer)
    // const ZERO: i8 = 0;
    // const THREE: i8 = 0;
    // const FIVE: i8 = 5;
    // const FIFTEEN: i8 = 15;
    // const HUNDRED: i8 = 100;
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz")
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count)
        }
        count += 1
    }
}
