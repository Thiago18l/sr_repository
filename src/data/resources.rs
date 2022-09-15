use crate::data::structs::User;
pub fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}