use crate::entity::RoleName;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleResponse {
    pub id: u64,
    pub name: RoleName,
    pub description: String,
    pub create_date: DateTime<Utc>,
}
