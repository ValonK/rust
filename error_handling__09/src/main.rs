// Rust doesn’t have exceptions. 
// Instead, it has the type Result<T, E> for recoverable errors and the panic!
//  macro that stops execution when the program encounters an unrecoverable error. 
use std::fs::File;

fn main() {
    read_file();
}

fn read_file(){
    let file = File::open("hello.txt");
    match file {
        Ok(f) => println!("file exists"), 
        Err(e) => println!("file does not exise"),
    };
}

fn trigger_panic(){
    let arr = vec![1,2,3];
    let m = arr[44];
}