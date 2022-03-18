// vecors - resizable arrays

pub fn run(){
    let numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[0]);


    // re-assign value
    let mut num2: Vec<i32> = vec![1,2,3,4,5];
    num2[2] = 20;

    // add on to vector
    num2.push(5);
    num2.push(6);

    // pop off last value
    num2.pop();

    println!("{:?}", num2);

    // get vector length

    println!("vector length: {}", num2.len());


    // get slice

    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);


    // loop through vector values

    for x in num2.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate value
    for x in num2.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", num2);
}