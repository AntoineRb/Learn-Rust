#![allow(unused)]

// Compound data types
// - tupple
// - array

fn main() {
    // Array - fixed length, known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[2] = {}", {arr[2]});

    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 4;
    println!("arr[1] = {}", {arr[1]});

    // Init an array with 10 items equals to 0;
    let arr: [u32; 10] = [0; 10];
    println!("initial array: {:?}", arr);

    // Slice - length not known at compile time
    // Slices are reference to a segment of array 
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];
    // First 3 elements
    let s = &nums[0..3];
    println!("Nums slice (fist 3 elements of nums array) {:?}", s);
    // First 4 elements
    let s = &nums[0..4];
    println!("Nums slice (fist 4 elements of nums array) {:?}", s);
    // Last 3 elements
    let s = &nums[7..];
    println!("Nums slice (Last 2 elements of nums array) {:?}", s);
}