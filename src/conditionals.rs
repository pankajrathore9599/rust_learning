// contionals - used to check the condition of somr=ething and act.


pub fn run(){
    let age: u8 = 23;
    let check_id: bool = true;
    let knows_person_of_age = true;


    // if/else

    if age >= 21 {
        println!("you are eligible in pub");
    } else {
        println!("sorry you are under age");
    }

    // checking id with valid age

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bar: you are eligible for enjoying drinks in pub");
    } else if age < 21 && check_id {
        println!("Bar: sorry you are under age");
    } else {
        println!("Bar: i need to see your ID");

    }

    // shorthand if

    let is_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_age)
}