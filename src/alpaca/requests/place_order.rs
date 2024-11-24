use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::alpaca::enums::{OrderClass, OrderType, PositionIntent, Side, StringEnum, TimeInForce};

#[derive(Serialize, Deserialize)]
pub struct PlaceOrder {
    pub symbol: String,
    pub qty: BigDecimal,
    pub notional: BigDecimal,
    pub side: StringEnum<Side>,
    #[serde(rename = "type")]
    pub order_type: StringEnum<OrderType>,
    pub time_in_force: StringEnum<TimeInForce>,
    pub limit_price: BigDecimal,
    pub extended_hours: bool,
    pub stop_price: BigDecimal,
    pub client_order_id: String,
    pub order_class: StringEnum<OrderClass>,
    pub take_profit: TakeProfit,
    pub stop_loss: StopLoss,
    pub trail_price: BigDecimal,
    pub trail_percent: BigDecimal,
    pub position_intent: StringEnum<PositionIntent>,
}

#[derive(Serialize, Deserialize)]
pub struct TakeProfit {
    pub limit_price: BigDecimal,
}
#[derive(Serialize, Deserialize)]
pub struct StopLoss {
    pub limit_price: BigDecimal,
    pub stop_price: BigDecimal,
}
