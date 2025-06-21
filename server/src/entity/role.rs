use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRoleEntity {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct RoleEntity {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub create_date: DateTime<Utc>,
}
