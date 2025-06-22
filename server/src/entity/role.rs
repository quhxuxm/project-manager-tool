use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::fmt::{Display, Formatter};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Copy, Type)]
#[repr(u32)]
pub enum RoleName {
    #[serde(rename = "sys-admin")]
    SysAdmin = 0,
    #[serde(rename = "org-admin")]
    OrgAdmin = 1,
    #[serde(rename = "user")]
    User = 2,
    #[serde(rename = "manager")]
    Manager = 3,
}
impl Display for RoleName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleName::SysAdmin => {
                write!(f, "sys-admin")
            }
            RoleName::OrgAdmin => {
                write!(f, "org-admin")
            }
            RoleName::User => {
                write!(f, "user")
            }
            RoleName::Manager => {
                write!(f, "manager")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRoleEntity {
    pub name: RoleName,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct RoleEntity {
    pub id: u64,
    pub name: RoleName,
    pub description: String,
    pub create_date: DateTime<Utc>,
}
