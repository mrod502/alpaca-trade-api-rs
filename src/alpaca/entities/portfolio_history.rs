use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::alpaca::enums::{StringEnum, TimeFrame};

#[derive(Serialize, Deserialize)]
pub struct PortfolioHistory {
    pub base_value: BigDecimal,
    pub equity: Vec<BigDecimal>,
    pub profit_loss: Vec<BigDecimal>,
    pub profit_loss_pct: Vec<BigDecimal>,
    pub timeframe: StringEnum<TimeFrame>,
    pub timestamp: Vec<i64>,
}
