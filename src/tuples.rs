
// tuples group together values of different types
// max 12 elements

pub fn run() {

    let person: (&str, &str, i8) = ("Pankaj","Sehore",27);
    
    println!("{} is from {} and is {}", person.0,person.1,person.2)
}