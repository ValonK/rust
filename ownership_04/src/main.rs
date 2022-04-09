use std::str::pattern::StrSearcher;


fn main() {
    ownership_fn_example();
}

fn scope_example() {

    let s1 = String::from("hello, world");
    let s2 = s1;

    // println!("s1={}, s2={}", s1, s2); will not compile, s1 is dropped
}

fn scope_example2() {

    let s1 = String::from("hello, world");
    let s2 = s1.clone();

    println!("s1={}, s2={}", s1, s2); // works because we copy
}

fn stack_only_copy(){

    // simple scalar types like integers, get copyied on to the stack
    let x = 1;
    let y = x;
    println!("x={}, y={}", x, y); 
}

fn ownership_fn_example(){

    let s1 = String::from("hello, world");


    // ownership_fn_example_child takes ownership of s1
    ownership_fn_example_child(s1); // s1 is moved into function
    // println!("s1 valu = {}", s1);              s1 is no longer valid here

    let x = 5;
    ownership_fn_example_int_child(x); // x would move into function, but x is i32 and is copy
    println!("x = {}", x);             // so its ok to use x afterwards

} // here x gets out of scope, then s, 
  //BUT because s's value was moved nothing special happens here for s

fn ownership_fn_example_child(some_string: String){ // some_string gets into scope
    println!("Value: {}", some_string);
} // here some_string gets out of scope, and drop() gets called and the memory freed.

fn ownership_fn_example_int_child(value: i32){ // value gets into scope
    println!("int value = {}", value);
} // value gets out of scope, but nothing happens here because value was copied



// return value ownership

fn gives_ownership_start(){

    let s1 = gives_ownership(); // gives_ownership moves its value into s1
    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.


fn gives_ownership() -> String {
    let some_string = String::from("hello, world");
    some_string // some string gets returned and moved into the calling function 
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}