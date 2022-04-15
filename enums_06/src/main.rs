fn main() {

    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
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