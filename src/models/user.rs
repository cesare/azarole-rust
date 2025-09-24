#[derive(Clone)]
pub struct User {
    id: u32,
}

impl User {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
