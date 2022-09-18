use crate::data::resources::build_user;

pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool
}

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn data_structs() {
    let user = User {
        username: String::from("Thiago"),
        email: String::from("thiago.lopes.dev@gmail.com"),
        sign_in_count: 1,
        active: true
    };
    println!("User Information:");
    print!("Username: {}\n", user.username);
    print!("E-mail: {}\n", user.email);
    print!("Active: {}\n", user.active);
    print!("Sign in counts: {}\n", user.sign_in_count);


    // Print user
    let user2: User = build_user(user.email, user.username);
    println!("{}, {}", user2.username, user2.email);


    // Tuples

    let rectangle: Rectangle = Rectangle { width: 25, height: 30 };
    println!("The area of the rectangle is {} square pixels", rectangle.area());

    let rect1 = Rectangle { width: 30, height: 20 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 20, height: 25 };
    println!("Can rect1 holds rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 holds rect3? {}", rect1.can_hold(&rect3));

    associated_func_impl();
}

// Associated functions

fn associated_func_impl() -> () {

    let rec = Rectangle::square(3);
    println!("width: {}, heigth: {}", rec.width, rec.height);
}