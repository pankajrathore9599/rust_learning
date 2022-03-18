// reference pointers - point to a resource in memory


pub fn run(){

    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

        //  with non primitive, if you assign another variable to a place of data, the first variable will no longer hold
        // that value. you'll need to use a reference (&) to point to resource

    println!("values: {:?}", (arr1,arr2));

    // vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("values: {:?}", (&vec1,vec2));


}