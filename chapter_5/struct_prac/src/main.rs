struct User {
    email:String,
    username:String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(email:String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");

    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("username02"),
        ..user1
    };

    println!("Hello {}, are you here {}? Your email is {}. You have been here {} many times.", user2.username, user2.active, user2.email, user2.sign_in_count);
    
}


