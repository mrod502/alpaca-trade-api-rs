use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use super::{asset_class::AssetClass, string_enum::StringEnum};

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub asset_id: String,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: StringEnum<AssetClass>,
    pub asset_marginable: bool,
    pub qty: BigDecimal,
    pub qty_available: BigDecimal,
    pub avg_entry_price: BigDecimal,
    pub side: String,
    pub market_value: BigDecimal,
    pub cost_basis: BigDecimal,
    pub unrealized_pl: BigDecimal,
    pub unrealized_plpc: BigDecimal,
    pub unrealized_intraday_pl: BigDecimal,
    pub unrealized_intraday_plpc: BigDecimal,
    pub current_price: BigDecimal,
    pub lastday_price: BigDecimal,
    pub change_today: BigDecimal,
}
