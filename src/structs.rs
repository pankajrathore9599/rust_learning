
// structs - used to create custom data types

// traditional struct
struct color {
    red: u8,
    green: u8,
    blue: u8
}


pub fn run() {
    let mut c = color {
        red: 255,
        green: 0,
        blue: 0
    };

    println! ("Color: {} {} {}", c.red, c.green, c.blue);
}
