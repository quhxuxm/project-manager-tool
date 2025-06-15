use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    id: u64,
    name: String,
    description: String,
    create_date: DateTime<Utc>,
    updated_date: DateTime<Utc>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    assignee_id: Option<u64>,
    parent_id: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Project,
    Story,
    Task,
    SubTask,
}
