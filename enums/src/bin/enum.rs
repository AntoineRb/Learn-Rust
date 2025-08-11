#![allow(unused)]

#[derive(Debug)]
#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}
fn main() {
    // Enum
    let red: Color = Color::Red;
    let green = Color::Green;
    let rgba = Color::Rgba(255, 255, 255, 0.8);
    let hex = Color::Hex("#ffffff".to_string());
    let hsl = Color::Hsl { h: 0, s: 1, l: 200 };
    // Attributes - Debug and PartialEq
    println!("{:?}", hsl);
    // ParitalEq
    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);
    // Option - An option in an enum that can take on two values: Some(T) or None
    let none_color: Option<Color> = None;
    let option_color: Option<Color> = Some(Color::Blue);
    println!("{:?}", none_color);
    println!("{:?}", option_color);
    // Result - Is like a special enum that can be used to represent either success (Ok(T)) or failure (Err(E))
    let result_color: Result<Color, String> = Ok(Color::Green);
    let error_color: Result<Color, String> = Err("Color not found".to_string());
    println!("{:?}", result_color);
    println!("{:?}", error_color);
}