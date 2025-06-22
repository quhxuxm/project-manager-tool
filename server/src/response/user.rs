use crate::entity::RoleName;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: u64,
    pub username: String,
    pub create_date: DateTime<Utc>,
    pub roles: Vec<RoleName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindUserResponse {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub create_date: DateTime<Utc>,
    pub roles: Vec<RoleName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateResponse {
    pub authentication_token: String,
}
