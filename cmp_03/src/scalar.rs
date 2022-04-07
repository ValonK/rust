pub fn mutable_example() {
    let mut number = 2;
    println!("number: {}", number);
    number = 3;
    println!("number: {}", number);
}

pub const CONST_VALUE : i32 = 10_000;

pub fn shadowing_example() {

    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value if x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let name = "Valon";
    {
        let name = "Adrian";
        println!("name: {}", name);
    }

    println!("name: {}", name);

    let spaces = "    ";
    let spaces = spaces.len();

    print!("Spaces: {}", spaces);
}

pub fn integer_types(){

    let i8_signed : i8 = -1;
    let u8_unsigned : u8 = 1;

    let i16_signed: i16 = -16;
    let u16_unsigned: u16 = 16;

    let i32_signed: i32 = -32;
    let u32_unsigned: u32 = 32;
    
    let i64_signed: i64 = -64;
    let u64_unsigned: u64 = 64;

    let i128_signed: i128 = -128;
    let u128_unsigned: u128 = 128;

    let i_size : isize = -100;
    let u_size: usize = 100;

    println!("8-bit signed: {}", i8_signed);
    println!("8-bit unsigned: {}", u8_unsigned);

    println!("16-bit signed: {}", i16_signed);
    println!("16-bit unsigned: {}", u16_unsigned);

    println!("32-bit signed: {}", i32_signed);
    println!("32-bit unsigned: {}", u32_unsigned);

    println!("64-bit signed: {}", i64_signed);
    println!("64-bit unsigned: {}", u64_unsigned);

    println!("128-bit signed: {}", u128_unsigned);
    println!("128-bit unsigned: {}", i128_signed);

    println!("isize: {}", i_size);
    println!("usize: {}", u_size); 
}

pub fn number_literal_examples(){

    let decimal : i32 = 98_222;
    let hex : i32 = 0xff;
    let octal : i32 = 0o77;
    let binary : i32 = 0b1111_0000;
    let byte : u8 = b'A';

    print!("Decimal: {}", decimal);
    print!("Hex: {}", hex);
    print!("Octal: {}", octal);
    print!("Binary: {}", binary);
    print!("Byte: {}", byte);
}

pub fn floatingtype_numbers(){
    let x = 2.0;
    let y: f32 = 1.0;
    println!("f64: {}", x);
    println!("f32: {}", y);
}

pub fn numeric_operations(){
    // addituin 
    let sum = 5 + 2;

    // subtraction
    let sub = 4 - 2;

    // multiplication 
    let mul = 4 * 2;

    // division 
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;

    // remainder
    let rem = 43 % 5;

    println!("Addition: {}", sum);
    println!("Subtraction : {}", sub);
    println!("Multiplikation : {}", mul);
    println!("Quotient : {}", quotient);
    println!("Floored : {}", floored);
    println!("Remainder : {}", rem);
}