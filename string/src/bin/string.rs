#![allow(unused)]

// String and &str
fn main() {
    // String is equal to vector of u8 (Vec<u8>) valid UTF-8
    // &str is equal to slice of u8 (&[u8]) valid UTF-8

    // When to use String vs &str
    // String -> mutate or data needs to be owned
    // &str -> read only

    // String
    let msg: String = String::from("Hello Rust"); 
    let len: usize = msg.len();
    println!("msg: {msg}");
    println!("msg length: {len}");

    // str - string slice
    // &str
    // - usually used str with reference (borrowed)
    // - immutable
    let msg: String = String::from("Hello Rust"); 
    let s: &str = &msg[0..5];
    let len: usize = s.len();
    println!("slice: {s}");
    println!("slice length: {len}");

    let hello: &str = "Hello";
    let s: &str = r#"
        {
            "a": 1,
            "b": {"c": 2},
            "d": 3,
        }
    "#;
    println!("Multiline string (json): {s}");

    // Deref coercion
    let msg: String = String::from("Hello Rust"); 
    let s: &str = &msg;

    // Add &str to a string
    let mut msg: String = "Hello Rust".to_string();
    msg += "!";
    println!("msg concat: {msg}");

    // String interpolation is a handy way to mix string literals with variables
    let lang = "Rust";
    let emoji = "🦀";
    let msg = format!("Hello {} {}", lang, emoji );
    println!("msg formatted: {msg}");

}