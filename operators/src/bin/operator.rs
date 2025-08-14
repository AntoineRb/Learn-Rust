#![allow(unused)]

fn main() {
    // +, -, *, /
    let a: i32 = 1;
    let b: i32 = 2;
    let c = a + b;
    println!("{a} + {b} = {c}");
    let c = a - b;
    println!("{a} - {b} = {c}");
    let c = a * b;
    println!("{a} * {b} = {c}");
    let c = a / b;
    println!("{a} / {b} = {c} (i32)");
    let c:f32 = a as f32 / b as f32;
    println!("{a} / {b} = {c} (f32)");

    // % (remainder != modulo operator)
    // mod
    // a % b = r, 0 <= r < b
    // -1 % 2 = 1
    // remainder
    // -1 % 2 = -11
    let a = -1;
    let b = 2;
    let rem = a % b;
    println!("{a} % {b} = {rem}");
    let a = 1;
    let b = 2;
    let rem = a % b;
    println!("{a} % {b} = {rem}");

    // Literals
    let a = 1i32;
    let b = 3u64;
    let c = 1.23e3; // 1.23 x 10^3 = 1230
    let d = 1_000_000_000u32;

    // Boolean
    let a = true && false;
    let a = true || false;
    let a = !true;

    // Bitwise
    // In binary the first number a will be equal to 101 
    // u8 is an unsigned 8-bit integer
    let a: u8 = 5; // 101
    let b: u8 = 3; // 011
    // & is bitwise AND
    println!("{a} & {b} = {:03b}", a & b); // 001
    // | is bitwise OR
    println!("{a} | {b} = {:03b}", a | b); // 111
    // ^ is bitwise XOR
    // XOR is exclusive OR, it returns 1 if bits are different
    // and 0 if they are the same
    println!("{a} ^ {b} = {:03b}", a ^ b); // 111
    println!("{a} ^ {a} = {:03b}", a ^ a); // 000
    // ! is bitwise NOT (inverts all bits)
    println!("!a = {:03b}", !a); // 11111010
    // << is left shift
    // >> is right shift
    // << and >> shift bits left or right
    // by the number of positions specified
    // in the right operand
    // 1 << 3 = 1000 (1 shifted left by 3 positions)
    println!("1 << 3 = {:03b}", 1u32 << 3); // 1000
    // 1 >> 3 = 000 (1 shifted right by 3 positions)
    println!("1 >> 3 = {:03b}", 1u32 >> 3); // 000
    // 10 >> 2 = 2 (10 in binary is 1010, shifted right by 2 positions is 0010)
    println!("10 >> 2 = {:03b}", 10u32 >> 2); // 0010

}