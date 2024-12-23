use crate::alpaca::enums::{StringEnum, TimeFrame};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetPortfolioHistory {
    pub period: Option<String>,
    pub time_frame: Option<StringEnum<TimeFrame>>,
    pub date_end: Option<DateTime<Utc>>,
    pub extended_hours: bool,
}
