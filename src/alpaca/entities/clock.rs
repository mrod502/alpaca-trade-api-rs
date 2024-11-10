use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Clock {
    pub timestamp: NaiveDateTime,
    pub is_open: bool,
    pub next_open: NaiveDateTime,
    pub next_close: NaiveDateTime,
}
