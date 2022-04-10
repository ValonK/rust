
fn main() {
    
    let user = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1
    };

    let user2 = User {
        username: user.username,
        ..user
    };

    // create instance of tuple struct
    let color = Color(1, 2, 3);
    let point = Point(1, 1, 1);

    let subject = AlwaysEqual;
}

fn create_user(username: String, email: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn create_user_short_hand(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
 

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
struct AlwaysEqual;