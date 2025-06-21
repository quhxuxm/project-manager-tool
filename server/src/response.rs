use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: u64,
    pub username: String,
    pub create_date: DateTime<Utc>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRuleResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub create_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStoryResponse {}
