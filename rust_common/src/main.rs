//normal structure
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("me"),
        email: String::from("mail@mail.com"),
        sign_in_count: 0,    
    };

    let user2 = User {
        active: false,
        email: String::from("mail"),
        ..user1
    };
    // we can access email bs we dont unpack/move it to new user 
    println!("{}", user1.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
}