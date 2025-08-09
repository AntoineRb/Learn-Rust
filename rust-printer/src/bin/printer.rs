#![allow(unused)]



#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}

fn main() {

    let lang = "Rust";
    let message = format!("Executing the {} printer!", lang);
    println!("message: \"{message}\"");

    let x = 5;
    println!("{0} x {0} = {1}", x, x * x);

    let lang = Lang {
        language: "Rust".to_string(),
        version: "1.3".to_string(),
    };
    println!("Rust object: {:?}", lang);
    println!("Rust object: {:#?}", lang);
}