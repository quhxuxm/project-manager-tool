use crate::entity::Role;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub create_date: DateTime<Utc>,
    pub role: Role,
}
