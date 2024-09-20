fn main() {
    let user1 = User {
        email: String::from("Saed@gmail.com"),
        username: String::from("Saed"),
        active: true,
        sign_in_count: 1
    };

    let mut user2 = User {
        email: String::from("alguem@exemplo.com"),
        username: String::from("algumnome123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("outroemail@exemplo.com");

    let user3 = User {
        email: String::from("outro@exemplo.com"),
        username: String::from("outronome567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };


    let user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
