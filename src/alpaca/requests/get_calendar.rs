use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetCalendar {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
