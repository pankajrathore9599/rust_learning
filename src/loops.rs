


pub fn run(){
    let mut count = 0;

    // infinite loop
    // loop {
    //     count += 1;
    //     println!("number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // while loop 

    while count <= 100 {
        if count % 15 == 0 {
            println!("my loop");
        } else if count % 3 == 0 {
            println!("my");
        } else if count % 5 == 0 {
            println!("loop");
        } else {
            println!("{}", count);
        }
    }

    // Include
    count += 1;
}