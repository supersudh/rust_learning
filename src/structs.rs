struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
// struct Color(u8, u8, u8);

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);
}