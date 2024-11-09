use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use super::{string_enum::StringEnum, time_frame::TimeFrame};

#[derive(Serialize, Deserialize)]
pub struct PortfolioHistory {
    pub base_value: BigDecimal,
    pub equity: Vec<BigDecimal>,
    pub profit_loss: Vec<BigDecimal>,
    pub profit_loss_pct: Vec<BigDecimal>,
    pub timeframe: StringEnum<TimeFrame>,
    pub timestamp: Vec<i64>,
}
