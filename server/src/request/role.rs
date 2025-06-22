use crate::entity::RoleName;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: RoleName,
    pub description: String,
}
