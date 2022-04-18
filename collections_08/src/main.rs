mod utf8strings;
mod hashmaps;
use std::vec;

fn main() {

    let mut v2 = vec![1,2,3,4];
    v2.push(2);
}

fn create_vec(){
    let v: Vec<i32> = Vec::new();
}

fn create_using_macro(){
    let v = vec![1,2,3];
}

fn add_val_to_vec(){
    let mut v = vec![1,2,3];
    v.push(1);
    v.push(2);
}

fn get_value_from_vec_with_index(){
    let mut v = vec![1,2,3];
    let third: &i32 = &v[2];  
}

fn get_value_from_vec_with_get(){

    let mut v = vec![1,2,3];
    match v.get(2){
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element"),
    }
}

fn iterate_immutable_vec(){
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }
}

fn iterate_mutable_vec(){
    let mut v = vec![1,2,3];
    for i in &mut v {
        *i += 50;
    }
}

fn vector_enum(){
    let row = vec![
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Float(10.23),
        SpreadsheetCell::Text(String::from("hi")),
    ];
}

enum SpreadsheetCell{
    Int(i32),
    Float(f32),
    Text(String),
}

