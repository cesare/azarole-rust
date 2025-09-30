use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct WorkplaceId(u32);
