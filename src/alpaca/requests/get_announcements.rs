use chrono::NaiveDateTime;

use crate::alpaca::enums::{DateType, StringEnum};

pub struct GetAnnouncements {
    pub ca_types: Vec<String>,
    pub since: NaiveDateTime,
    pub until: NaiveDateTime,
    pub symbol: String,
    pub cusip: String,
    pub date_type: StringEnum<DateType>,
}
