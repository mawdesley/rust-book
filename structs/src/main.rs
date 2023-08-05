struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user (email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(String::from("chrus@gmail.com"), String::from("chrus"));
    user1.sign_in_count +=1;

    println!("user1: {}", user1.username);

    let user2 = User {
        email: String::from("chrus2@gmail.com"),
        ..user1
    };


    let c = Color(127, 150, 0);
    println!("Color: {}, {}, {}", c.0, c.1, c.2);

    let subject = AlwaysEqual;
}
