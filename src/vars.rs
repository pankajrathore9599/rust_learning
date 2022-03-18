// Variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let name = "Pankaj";
    let age = 27;

    println!("My name  is {} and my age is {}", name, age);

    // Define constant
    const ID : i32 = 001;
    println!("ID: {}", ID);

    // assign multiple varibles
    let (my_name, my_age) = ("Pankaj",27);
    println!("{} is {}", my_name,my_age);
}