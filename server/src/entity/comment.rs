use crate::entity::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    id: u64,
    content: String,
    create_date: DateTime<Utc>,
    update_date: DateTime<Utc>,
    author: User,
    item_id: u64,
}
