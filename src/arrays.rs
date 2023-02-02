use std::{mem, slice};

pub fn run() {
    // let numbers: [i32;5] = [1, 2, 3, 4, 5,];
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //for single value
    println!("Single Value : {}", numbers[0]);

    //get array length
    println!("Arrays length : {}", numbers.len());

    //arrays are stack allocates
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice from array
    // let slice: &[i32] = &numbers;
    let slice: &[i32] = &numbers[2..4];
    println!("Slice :{:?}", slice);
}
