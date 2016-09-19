#![allow(overflowing_literals)]

fn basic_casting_examples() {
    let decimal = 65.4321_f32;
    // cannot do implicit conversion
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u16 is: {}", 1000 as u8);
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    println!("1000 mod 256 is : {}", 1000 % 256);

    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is:  {}", 128 as i8);
}

fn literals() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn main() {
    basic_casting_examples();
    literals();
}
