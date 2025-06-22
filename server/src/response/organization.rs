use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrganizationResponse {
    pub name: String,
    pub description: String,
    pub create_date: DateTime<Utc>,
}
