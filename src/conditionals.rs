pub fn run() {
    let age = 30;
    let is_greater_than_30: bool = false;
    let priviledged_member = true;

    // If/Else
    if age >= 30 && is_greater_than_30 || priviledged_member {
        println!("Age is greater than 30")
    } else if age < 30 && is_greater_than_30 {
        println!("Age is lesser than 30")
    } else {
        println!("Wait, What?")
    }

    // Shorthand if
    let is_of_age = if age >= 30 { true } else { false };
    println!("Is of Age: {}", is_of_age)
}