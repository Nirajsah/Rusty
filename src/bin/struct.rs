#![allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 0,
            active: false,
        }
    }
}

fn main() {
    let mut user = User::new(
        String::from("Godzilla"),
        String::from("godzilla@monarch.com"),
    );
    user.active = true;
    user.sign_in_count = 1;
    println!("{:?}", user.username);
}
