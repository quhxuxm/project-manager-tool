use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrganizationEntity {
    pub name: String,
    pub description: String,
    pub creator_id: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct OrganizationEntity {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub create_date: DateTime<Utc>,
}
