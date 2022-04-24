use std::{fs::File, io::{ErrorKind, Error, Read}};
use core::panic;

// Rust doesn’t have exceptions. 
// Instead, it has the type Result<T, E> for recoverable errors and the panic!
//  macro that stops execution when the program encounters an unrecoverable error. 

const FILE_PATH: &'static str = "hello.txt";

fn main() {
    let content = read_content_from_file_even_shorter_version();
    println!("Content: {}", content.unwrap());
}

fn read_file(){

    let file = File::open(FILE_PATH);

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_PATH) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn open_file_unwrap(){
    // unwrap will return the result if the Result<T, E> is OK, if there is an Err unwrap will panic
    let file = File::open(FILE_PATH).unwrap();

    // with expect we can specify the message for panic
    let file2 = File::open(FILE_PATH).expect("Problem opening file");
}

fn trigger_panic(){
    let arr = vec![1,2,3];
    let m = arr[44];
}

// Propagating Errors
fn read_content_from_file() -> Result<String, Error> {
    let f = File::open(FILE_PATH);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)        
    };

    let mut content = String::new();
    match f.read_to_string(&mut content){
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

// The ? placed after a Result value is defined to work in almost the same way as the match expressions
// we defined to handle the Result values.
// If the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
// and the program will continue. If the value is an Err, the Err will be returned from the whole function 
// as if we had used the return keyword so the error value gets propagated to the calling code.
fn read_content_from_file_short_version() -> Result<String, Error>{
    let mut file = File::open(FILE_PATH)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn read_content_from_file_even_shorter_version() -> Result<String, Error> {
    let mut content = String::new();
    File::open(FILE_PATH)?.read_to_string(&mut content)?;
    Ok(content)
}