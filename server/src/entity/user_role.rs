use chrono::{DateTime, Utc};
pub struct CreateUserRoleEntity {
    pub user_id: u64,
    pub role_id: u64,
}

pub struct UserRoleEntity {
    pub user_id: u64,
    pub role_id: u64,
    pub create_date: DateTime<Utc>,
}
