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

fn main() {
    basic_casting_examples();
}
