mod matchexample;

fn main() {

    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let a = Message::Move {x: 12, y: 12};


    matchexample::value_in_cents_us(
        matchexample::UsCoins::Quarter(
            matchexample::UsState::Alaska));
}

// struct Test {
    
// }

// We can put any type of data as param
// enum IpAddrKind {
//     V4(Test),
//     V6(String),
// }


// multiple params, for different enum types
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// function enum, wich can take parameters
//enum IpAddrKind {
//    V4(String),
//    V6(String),
//}

// normal enum
// enum IpAddrKind {
//     V4,
//     V6,
// }

enum Message {
    Quit, 
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32), 
}

// we’re also able to define methods on enums
impl Message {
    fn call(&self){

    }    
}