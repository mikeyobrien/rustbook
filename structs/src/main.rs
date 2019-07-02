struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    let user1 = build_user("dude@email.com".to_string(), "dude".to_string());

    println!("{}\n{}\n{}\n{}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let user2 = User {
        email: String::from("dude2@example.com"),
        username: String::from("anotheruser"),
        ..user1
    };

    println!("{}\n{}\n{}\n{}", user2.email, user2.username, user2.active, user2.sign_in_count);
}

