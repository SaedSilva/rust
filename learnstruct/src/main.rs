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

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let length1 = 50;
    let width1 = 30;

    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area2((length1, width1))
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    let rect4 = Rectangle::square(50);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.length > rectangle.length && self.width > rectangle.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
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

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
