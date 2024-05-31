use std::fmt;

struct User {
    username: String,
    active: bool,
    email: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Username: {}\nEmail: {}\nActive: {}\nSign in count: {}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}

fn main() {
    let user1 = User {
        email: String::from("hello"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 5,
    };

    println!("user1 {}", user1);

    let user2 = build_user(
        String::from("ankit@gmail.com"),
        String::from("ankit chotaliya"),
    );

    println!("user2 {}", user2);

    let user3 = User {
        email: String::from("ankit2@gmail.com"),
        username: String::from("ankit2 chotaliya"),
        ..user1
    };

    println!("user3 {}", user3)
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
