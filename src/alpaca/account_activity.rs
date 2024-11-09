use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountActivity {
    pub id: String,
    pub activity_type: String,
    pub transaction_time: NaiveDateTime,
    #[serde(alias = "type")]
    pub account_type: String,
    pub price: BigDecimal,
    pub qty: BigDecimal,
    pub side: String,
    pub symbol: String,
    pub leaves_qty: BigDecimal,
    pub cum_qty: BigDecimal,
    pub date: NaiveDateTime,
    pub net_amount: BigDecimal,
    pub description: String,
    pub per_share_amount: BigDecimal,
    pub order_id: String,
    pub order_status: String,
    pub status: String,
}
