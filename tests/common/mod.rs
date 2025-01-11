#[derive(Debug)]
pub struct TestUser {
    pub username: String,
    pub age: f64,
    pub is_active: bool,
}

impl TestUser {
    pub fn new(username: &str, age: f64, is_active: bool) -> Self {
        Self {
            username: username.to_string(),
            age,
            is_active,
        }
    }
}