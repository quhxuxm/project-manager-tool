use crate::entity::RoleName;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub roles: Vec<RoleName>,
    pub organization: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateRequest {
    pub username: String,
    pub password: String,
}
