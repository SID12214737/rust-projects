struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

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

    println!("{}", user1.email);
}