use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Manager,
    Engineer,
    Administrator,
}
