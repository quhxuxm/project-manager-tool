use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRuleResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub create_date: DateTime<Utc>,
}
