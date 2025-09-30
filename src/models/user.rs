#[derive(Clone, Copy, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct UserId(u32);

#[derive(Clone)]
pub struct User {
    pub id: UserId,
}

impl User {
    pub fn new(id: UserId) -> Self {
        Self { id }
    }
}
