use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCommentEntity {
    id: u32,
    content: String,
    create_date: DateTime<Utc>,
    update_date: DateTime<Utc>,
    item_id: u64,
    user_id: u64,
}
