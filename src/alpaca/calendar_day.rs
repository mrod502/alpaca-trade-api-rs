use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CalendarDay {
    pub date: String,
    pub open: String,
    pub close: String,
}
