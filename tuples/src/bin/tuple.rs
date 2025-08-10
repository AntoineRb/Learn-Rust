#![allow(unused)]

fn main() {
    // Tuple
    let tup: (bool, u32, char) = (true, 1, 'A');
    // Destructure
    let (a, b, c) = tup;
    // Ignore with _
    let (_, e, _) = tup;
    // Empty tuple - unit type
    let empty_tup = ();
    // nested tuple
    let nested_tup = ((false, 5, 'R'), (true, 2, 'H'), (true, 3, 'P'));
    // Access to tuple items 
    println!("Nested tuple 1st item: {:#?}", nested_tup.0);
    println!("Nested tuple item 1st val: {}", nested_tup.0.0);

    let t: (char, char, char) = ('A', 'B', 'C');
    println!("Tuple T values: {}, {}, {}", t.0, t.1, t.2);


}