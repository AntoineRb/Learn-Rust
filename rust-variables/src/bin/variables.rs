#![allow(unused)]

// Variables are immutable by default 
// If i want to make it mutable i need to declare the variable with the keyword "mut"

// Type is usually inferred by rust so i don't have to declare the type of the variable
// I can also create a constant and i can also redeclare the same variable with a different type and a different value 
// This is called shadowing when we're dealing with generic types 

// And lastly we looked a type placeholder it's useful 
fn main() {
    let x: i32 = -123;
    // This will not compile
    // x += 1;

    let mut y: i32 = 123;
    y += 1;

    // Rust infer the type by looking at the value
    let z = -123;

    // This will not compile (unexpected type)
    // let w: () = 123;

    const NUM: u32 = 1;

    let x: i32 = -1;
    let x: bool = true;

    //  Use _ to infer the type of vector items
    let v: Vec<_> = vec![1, 2, 3];
    // Define a i32 vector 
    let v: Vec<i32> = vec![1, 2, 3];
}