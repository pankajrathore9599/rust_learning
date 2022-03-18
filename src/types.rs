/*
Primitive types -
Integers: u8, i8, u16, u32, i32, u64, u128, i128 (number of bits they tale in memory)
floats: f32, f64
boolean (bool)
chracters (char)
tuples
arrays
*/



pub fn run() {

    // default is i32
    let x = 1;

    // deefault is "f64"
    let y = 3.87;

    // add explict type
    let z: i64 = 8769780800;

    // find max size
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);

    // boolean

    let is_active = true;


    // get boolean from expresssion
    let is_greater: bool = 10 < 5;


    // emoji 
    let face = '\u{1F600}';


    // print all
    println!("{:?}", (x, y, z, is_active, is_greater, face));




}