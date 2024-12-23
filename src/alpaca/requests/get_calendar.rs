use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetCalendar {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}
