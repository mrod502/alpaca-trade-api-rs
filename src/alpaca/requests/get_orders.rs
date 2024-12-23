use chrono::{DateTime, Utc};

pub struct GetOrders {
    pub status: Option<String>,
    pub limit: Option<isize>,
    pub after: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
    pub direction: Option<String>,
    pub nested: Option<bool>,
    pub side: Option<String>,
    pub symbols: Option<Vec<String>>,
}
