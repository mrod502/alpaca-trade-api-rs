use chrono::{DateTime, Utc};

use crate::alpaca::enums::{DateType, StringEnum};

pub struct GetAnnouncements {
    pub ca_types: Vec<String>,
    pub since: DateTime<Utc>,
    pub until: DateTime<Utc>,
    pub symbol: Option<String>,
    pub cusip: Option<String>,
    pub date_type: StringEnum<DateType>,
}
