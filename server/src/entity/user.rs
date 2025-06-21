use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CreateUserEntity {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct UserEntity {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub create_date: DateTime<Utc>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct UserWithRoleNameEntity {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub create_date: DateTime<Utc>,
    pub role_name: String,
}
