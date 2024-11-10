use bigdecimal::BigDecimal;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::alpaca::enums::{AssetClass, OrderClass, OrderType, Side, StringEnum, TimeInForce};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub client_order_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub submitted_at: NaiveDateTime,
    pub filled_at: NaiveDateTime,
    pub expired_at: NaiveDateTime,
    pub canceled_at: NaiveDateTime,
    pub failed_at: NaiveDateTime,
    pub replaced_at: NaiveDateTime,
    pub replaced_by: String,
    pub replaces: String,
    pub asset_id: String,
    pub symbol: String,
    pub asset_class: StringEnum<AssetClass>,
    pub order_class: StringEnum<OrderClass>,
    #[serde(alias = "type")]
    pub order_type: StringEnum<OrderType>,
    pub side: StringEnum<Side>,
    pub time_in_force: StringEnum<TimeInForce>,
    pub status: String,
    pub notional: BigDecimal,
    pub qty: BigDecimal,
    pub filled_qty: BigDecimal,
    pub filled_avg_price: BigDecimal,
    pub limit_price: BigDecimal,
    pub stop_price: BigDecimal,
    pub trail_price: BigDecimal,
    pub trail_percent: BigDecimal,
    pub hwm: BigDecimal,
    pub extended_hours: bool,
    pub legs: Vec<Order>,
}
