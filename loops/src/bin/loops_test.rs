#![allow(unused)]

fn main() {
    // loop
    let mut i = 0;
    loop {
        if (i > 5) {
            break;
        }
        println!("loop {i}");
        i += 1;
    }
    // while
    let mut i = 0;
    while i <= 3 {
        println!("while {i}");
        i += 1;
    }
    // for loop
    for i in 0..5 {
        println!("for {i}");
    }
    for i in 0..=5 {
        println!("for 0 to 5: {i}");
    }
    // for loop array
    let arr = [1, 2, 3];
    for i in arr {
        println!("for loop arr {i}");
    }
    // usize and range
    let n: usize = arr.len();
    for i in 0..n {
        println!("array {}", arr[i]);
    }
    // for loop vector
    // iter
    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("vec {x}");
    }
    for x in v.iter() {
        println!("vec {x}");
    }
    // Retun value
    let mut i = 0;
    let z = loop {
        if i == 3 {
            break 99;
        }
        i += 1;
    };
    println!("return loop {z}");
    // labels
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            println!("{i}, {j}");
            if i == 1 && j == 2 {
                break 'outer;
            }
        }
    }
}