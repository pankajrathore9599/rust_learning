pub fn run() {
    // print in console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("Number: {} {}",123, 5675);

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}","Pankaj","Sehore","Code");

    // Named Arguments
    println!("{name} likes to play {activity}", name="Pankaj",activity="Football");

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    // placeholder for debug trait
    println!("{:?}",(12,true,"hey"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}