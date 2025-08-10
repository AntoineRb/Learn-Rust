/**
 *  The scalars types 
 *  Int: : i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
 *  Float: f32, f64
 *  Boolean: bool
 *  Chars: char
 * 
 *  For more information on the scalar types, see the Rust documentation on [scalar types](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types).
 */

fn main () {
    // Int signed
    // For more information on the integer types, see the Rust documentation on [integer types](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types).
    let i0: i8 = 0; // -128 to 127
    let i1: i16 = 1; // -32 768 à 32 767
    let i2: i32 = 1; // -2,1×10⁹
    let i3: i64 = 1; // bigest numbers
    let i4: i128 = 1; // bigest bigest bigest numbers
    let i5: isize = 1; // 32 or 64 bits based on computer archi
    // Int unsigned
    let i0: u8 = 0; // 0 à 255
    let i1: u16 = 1; // 0 à 65 535
    let i2: u32 = 1; // 2,1×10⁹
    let i3: u64 = 1; // bigest numbers
    let i4: u128 = 1; // bigest bigest bigest numbers
    let i5: usize = 1; // 32 or 64 bits based on computer archi
    // For more information on the floating-point types, see the Rust documentation on [floating-point types](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types).
    // Float
    let pi: f64 = 3.14159;
    let size = 72.5; // f64 by default
    // Boolean
    // For more information on the boolean type, see the Rust documentation on [boolean types](https://doc.rust-lang.org/book/ch03-02-data-types.html#boolean-types).
    let is_rust: bool = true;
    // Char
    // For more information on the char type, see the Rust documentation on [char types](https://doc.rust-lang.org/book/ch03-02-data-types.html#char-types).
    let character: char = 'A';
    // Type conversion
    // For more information on type conversion, see the Rust documentation on [type conversion](https://doc.rust-lang.org/book/ch03-02-data-types.html#type-conference).
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);
    // Min and Max
    // For more information on the min and max values of the scalar types, see the Rust documentation on [min and max values](https://doc.rust-lang.org/book/ch03-02-data-types.html#minimum-and-maximum-values).
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;
    println!("i32 min: {min_i}");
    println!("i32 max: {max_i}");

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;
    println!("char min: {min_char}");
    println!("char max: {max_char}");

    // overflow
    // For more information on overflow handling, see the Rust documentation on [overflow handling](https://doc.rust-lang.org/book/ch03-02-data-types.html#overflow-handling).
    let mut u: u32 = u32::MAX;
    u += 1;
    println!("overflow u32: {u}");
    // wrapping_* → wrap-around volontaire
    // For more information on the wrapping methods, see the Rust documentation on [wrapping methods](https://doc.rust-lang.org/std/primitive.u8.html#method.wrapping_add).
    // wrapping_add, wrapping_sub, wrapping_mul, wrapping_div, wrapping_rem
    // wrapping_* methods will wrap around the value if it overflows.
    // For example, if you add 1 to 255, it will wrap around to 0.
    // wrapping_add → wrap around the value
    let x: u8 = 255;
    let y = x.wrapping_add(1); // y = 0
    // checked_* → return None if overflow
    // For more information on the checked methods, see the Rust documentation on [checked methods](https://doc.rust-lang.org/std/primitive.u8.html#method.checked_add).
    // checked_add, checked_sub, checked_mul, checked_div, checked_rem
    // checked_* methods will return None if the operation would overflow.
    // For example, if you add 1 to 255, it will return None.
    // checked_add → return None if overflow
    let x: u8 = 255;
    let y = x.checked_add(1); // None
    // overflowing_* → return (result; bool overflow)
    // For more information on the overflowing methods, see the Rust documentation on [overflowing methods](https://doc.rust-lang.org/std/primitive.u8.html#method.overflowing_add).
    // overflowing_add, overflowing_sub, overflowing_mul, overflowing_div, overflowing_rem
    // overflowing_* methods will return a tuple with the result and a boolean indicating if the operation overflowed.
    // For example, if you add 1 to 255, it will return (0, true).
    // overflowing_add → return (result, overflow)
    let x: u8 = 255;
    let (res, overflow) = x.overflowing_add(1); // (0, true)
    // saturating_* → block the value to max/min
    // For more information on the saturating methods, see the Rust documentation on [saturating methods](https://doc.rust-lang.org/std/primitive.u8.html#method.saturating_add).
    // saturating_add, saturating_sub, saturating_mul, saturating_div, saturating_rem
    // saturating_* methods will return the maximum or minimum value if the operation would overflow.
    // For example, if you add 10 to 255, it will return 255.
    // saturating_add → block the value to max
    let x: u8 = 255;
    let y = x.saturating_add(10); // y = 255
    // overflow with signed integers
    // For more information on overflow with signed integers, see the Rust documentation on [overflow with signed integers](https://doc.rust-lang.org/book/ch03-02-data-types.html#overflow-with-signed-integers).
    // In Rust, signed integers will panic on overflow in debug mode and wrap around in release mode.
    // For example, if you add 1 to 127, it will panic in debug mode and wrap around to -128 in release mode.
    // To enable overflow checks in release mode, you can use the `-C overflow-checks=on` flag.
    let x: i8 = 127;
    let y = x.wrapping_add(1); // y = -128
}