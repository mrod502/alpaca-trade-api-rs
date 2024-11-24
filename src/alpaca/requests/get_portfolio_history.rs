use crate::alpaca::enums::{StringEnum, TimeFrame};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetPortfolioHistory {
    pub period: String,
    pub time_frame: StringEnum<TimeFrame>,
    pub date_end: NaiveDateTime,
    pub extended_hours: bool,
}
