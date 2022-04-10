// The Rules of References
// At any given time, you can have either one mutable reference 
// or any number of immutable references.
// References must always be valid.


pub fn references_example(){

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize{
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// When functions have references as parameters instead of the actual values,
// we won’t need to return the values in order to give back ownership, 
// because we never had ownership.

// We call the action of creating a reference borrowing. 
// As in real life, if a person owns something, you can borrow it from them.
// When you’re done, you have to give it back. You don’t own it.

fn modify_borrowed_example(){
    let s = String::from("hello");
    modify_borrowed_example_child(&s);
}

fn modify_borrowed_example_child(s: &String){
    // s.push_str(", world"); // doesn't work, references are immutable by default
}

fn modify_borrowed_example_mutable(){
    let mut s = String::from("hello");
    modify_borrowed_example_mutable_child(&mut s);
}

fn modify_borrowed_example_mutable_child(s: &mut String){
    s.push_str(", world")
}

// Mutable references have one big restriction: 
// you can have only one mutable reference to a particular piece of data at a time.
// This code that attempts to create two mutable references to s will fail:
fn mutable_refernce_restriction(){

    // Error: cannot borrow `s` as mutable more than once at a time
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut 2;
    println!("{}, {}", r1, r2);
}

// We also cannot have a mutable reference while we have an immutable one to the same value. 
// Users of an immutable reference don’t expect the value to suddenly change out from under them! 
// However, multiple immutable references are allowed because 
// no one who is just reading the data has the ability to affect anyone else’s reading of the data.
fn mutable_reference_restriction_scope_example(){

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn dangle_example() {
    let reference_to_nothing = no_dangle();
}

//fn dangle() -> &String { dangle returns a reference to a String

//   let s = String::from("hello"); s is a new String

//    &s  we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

