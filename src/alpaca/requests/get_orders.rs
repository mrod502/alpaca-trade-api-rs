use chrono::NaiveDateTime;

pub struct GetOrders {
    pub status: String,
    pub limit: isize,
    pub after: NaiveDateTime,
    pub until: NaiveDateTime,
    pub direction: String,
    pub nested: bool,
    pub side: String,
    pub symbols: Vec<String>,
}
