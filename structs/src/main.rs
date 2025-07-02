#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User {
        email: String::from("pavel@gmail.com"),
        username: String::from("pasha1367"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    println!("Username: {}", name);
    user1.username = String::from("pavel_changed");
    println!("Username: {}", name);
    println!("User1 {:?}", user1);

    let user2 = build_user(String::from("pavel@mail.ru"), String::from("NIkita22"));
    let user3 = User {
        email: String::from("Johndole@gmail.com"),
        username: String::from("james22"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
