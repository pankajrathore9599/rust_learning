//


pub fn run() {
    greeting("Hello", "Pankaj");

    // bind function value to variable

    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);
}

fn greeting(greet: &str, name: &str) {

    println!("{} {}, nice to meet you", greet, name);

}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
