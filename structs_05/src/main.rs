mod rectexample;
mod user;

// Structs let you create custom types that are meaningful for your domain.
// By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear.
// In impl blocks, you can define functions that are associated with your type,
// and methods are a kind of associated function that let you specify the behavior that instances of your structs have.

fn main() {
    
    let rect1 = rectexample::Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = rectexample::Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = rectexample::Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = rectexample::Rectangle::square(20);

    println!("Umfang: {}", rect1.cirumconference());

}

fn tuple_struct(){
    // create instance of tuple struct
    let color = Color(1, 2, 3);
    let point = Point(1, 1, 1);
}

fn unit_struct(){
    let subject = AlwaysEqual;
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
struct AlwaysEqual;