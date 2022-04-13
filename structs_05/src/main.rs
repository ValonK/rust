mod rectexample;
mod user;
use crate::user::User;
fn main() {
    
    // create instance of tuple struct
    let color = Color(1, 2, 3);
    let point = Point(1, 1, 1);

    let subject = AlwaysEqual;

    let rect = rectexample::Rectangle {
        height: 30,
        width: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Rectangle: width: {}", rect.width());


    let valon = User{
        name: String::from("Valon"),
        last_name: String::from("Kastrati"),
        email: String::from("v@vv.vv"), 
        is_active: true,
        password: String::from("123456")
    };

    // let full_name = valon.get_full_name();
    // println!("Full Name: {}", full_name);
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
struct AlwaysEqual;