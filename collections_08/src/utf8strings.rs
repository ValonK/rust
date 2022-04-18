// We discuss strings in the context of collections because strings are implemented as a collection of bytes,
// plus some methods to provide useful functionality when those bytes are interpreted as text. 

// Rust’s standard library also includes a number of other string types, such as OsString, OsStr, CString, and CStr.

// A String is a wrapper over a Vec<u8>

fn to_string_example(){

    let data = String::new();
    let s = data.to_string();
    let literal_str = "initial contents".to_string();
    let literal_str1  = String::from("initial contents"); // same as 11
}

fn strings_are_utf8_encoding(){
    let hello = String::from("السلام عليكم");
    let hello1 = String::from("Dobrý den");
    let hello2 = String::from("Hello");
    let hello3 = String::from("שָׁלוֹם");
    let hello4 = String::from("नमस्ते");
    let hello5 = String::from("こんにちは");
    let hello6 = String::from("안녕하세요");
    let hello7 = String::from("你好");
    let hello8 = String::from("Olá");
    let hello9 = String::from("Здравствуйте");
    let hello10 = String::from("Hola");
}

fn appending_str_push(){
    let mut s = String::from("foo");
    s.push('b');
    s.push('a');
    s.push('r');
}

fn appending_str_push_str(){
    let mut a = String::from("foo");
    a.push_str("bar");

    println!("{}", a);
}

// Concatenation with the + Operator or the format! Macro

// let s3 = s1 + &s2; looks like it will copy both strings and create a new one,
// this statement actually takes ownership of s1, appends a copy of the contents of s2,
// and then returns ownership of the result. I
fn concatinate_example(){
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);
}

fn concatinate_with_format_macro_example(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}

fn internal_string_representation(){

    // In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes long. 
    // Each of these letters takes 1 byte when encoded in UTF-8. 
    let hello = String::from("Hola");

    // Asked how long the string is, you might say 12.
    // In fact, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, 
    // because each Unicode scalar value in that string takes 2 bytes of storage. 
    // Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.
    let hello = String::from("Здравствуйте");
}

fn string_slice_example(){

    // use ranges to create string slices with caution, because doing so can crash your program.
    let hello = "Здравствуйте";
    let s = &hello[0..4];
}

fn iterate_with_chars_example(){
    for c in "Hello".chars(){
        println!("{}", c);
    }
}

fn iterate_with_bytes_example(){
    for b in "Hello".bytes(){
        println!("{}", b);
    }
}