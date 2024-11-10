use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::alpaca::entities::Order;

#[derive(Serialize, Deserialize)]
pub struct TradeUpdate {
    pub at: NaiveDateTime,
    pub event: String,
    pub event_id: String,
    pub execution_id: String,
    pub order: Order,
    pub position_qty: BigDecimal,
    pub price: BigDecimal,
    pub qty: BigDecimal,
    pub timestamp: NaiveDateTime,
}
