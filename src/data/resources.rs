use crate::data::structs::User;
use crate::data::structs::Rectangle;
pub fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}