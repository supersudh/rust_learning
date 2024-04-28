pub fn run() {
    // let mut count = 0;

    // ∞ loop

    // loop {
    //     count += 1;
    //     println!("Number {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While loop (buzzer)
    // const ZERO: i8 = 0; DOESN'T WORK WHY??? TODO
    // const THREE: i8 = 0; DOESN'T WORK WHY??? TODO
    // const FIVE: i8 = 5; DOESN'T WORK WHY??? TODO
    // const FIFTEEN: i8 = 15; DOESN'T WORK WHY??? TODO
    // const HUNDRED: i8 = 100; DOESN'T WORK WHY??? TODO
    // while count <= 100 { // [START]WORKS
    //     if count % 15 == 0 {
    //         println!("fizzbuzz")
    //     } else if count % 3 == 0 {
    //         println!("fizz");
    //     } else if count % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", count)
    //     }
    //     count += 1
    // } // [END] WORKS

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz")
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x)
        }
    }
}
