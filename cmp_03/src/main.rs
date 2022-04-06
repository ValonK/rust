fn main() {
    shadowing_example();
    mutable_example();
    print!("CONST: {}", CONST_VALUE);
    integer_types();
    number_literal_examples();
}


fn mutable_example() {
    let mut number = 2;
    println!("number: {}", number);
    number = 3;
    println!("number: {}", number);
}

const CONST_VALUE : i32 = 10_000;

fn shadowing_example() {

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

fn integer_types(){

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

fn number_literal_examples(){

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