// Primitive str = Immutable fixed-length string somewhere in memory
// string = growable, heap allocated data structure - use when you need to modify or own

pub fn run() {
    
    let mut hello = String::from("Hey ");

    // get length

    println!("{}",hello.len());

    // add something in current string, also use 'mut' keyboard for making string mutable.

    hello.push('G');   // for single chracter

    hello.push_str("Pankaj"); // for pushing string or more than one chracter.

    // capacity in bytes
    
    println!("Capacity: {}", hello.capacity());

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}",word);
    }

    println!("{}",hello);

}