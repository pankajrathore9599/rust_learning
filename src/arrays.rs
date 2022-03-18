
// arrays - fixed list where elements are the same data types
// use 'mut' for edit the array with CURD operations.

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[0]);


    // re-assign value
    let mut num2:[i32; 5] = [1,2,3,4,5];
    num2[2] = 20;

    println!("{:?}", num2);

    // get array length

    println!("array length: {}", num2.len());


    // get slice

    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

}