use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetAccountActivities {
    pub activity_types: Vec<String>,
    pub date: NaiveDateTime,
    pub until: NaiveDateTime,
    pub after: NaiveDateTime,
    pub direction: String,
    pub page_size: usize,
    pub page_token: String,
    pub category: String,
}
