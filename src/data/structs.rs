use crate::data::resources::build_user;


pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool
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
}