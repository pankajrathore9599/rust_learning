
// cli is used to run and create commands within the project


use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("Command: {:?}", command);
}