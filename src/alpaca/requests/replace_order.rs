use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::alpaca::enums::{StringEnum, TimeInForce};

#[derive(Serialize, Deserialize, Clone)]
pub struct ReplaceOrder {
    pub qty: BigDecimal,
    pub limit_price: BigDecimal,
    pub stop_price: BigDecimal,
    pub trail: BigDecimal,
    pub time_in_force: StringEnum<TimeInForce>,
    pub client_order_id: String,
}
