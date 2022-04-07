pub fn tuple_example(){
    let tuple1: (i32, f64, u8, char) = (-2, 3.2, 4, 'A');

    // Deconstruction
    let (integer, double, unsignedint, charC ) = tuple1;
    println!("Integer: {}", integer);
    println!("Double: {}", double);
    println!("Unsigned Int: {}", unsignedint);
    println!("Char: {}", charC);

    // directly accessing tuple values
    let integer_val = tuple1.0;
    let double_val = tuple1.1;
    let unsigned_val = tuple1.2;
    let char_val = tuple1.3;
}

pub fn array_example(){
    let number_array = [0, 1, 2, 3, 4, 5, 6];
    let str_array = ["0", "1", "2", "3", "4", "5", "6"];
    let char_array = ['0', '1', '2', '3', '5', '6'];

    let int_val: i32 = number_array[0];

    // create array with size & type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // create array & fill with same values
    let arr1 = [4; 4];
}