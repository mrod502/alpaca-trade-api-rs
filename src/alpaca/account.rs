use bigdecimal::BigDecimal;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub account_number: String,
    pub status: String,
    pub crypto_status: String,
    pub currency: String,
    pub buying_power: BigDecimal,
    pub regt_buying_power: BigDecimal,
    pub daytrading_buying_power: BigDecimal,
    pub effective_buying_power: BigDecimal,
    pub non_marginable_buying_power: BigDecimal,
    pub bod_dtbp: BigDecimal,
    pub cash: BigDecimal,
    pub accrued_fees: BigDecimal,
    pub portfolio_value: BigDecimal,
    pub pattern_day_trader: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
    pub account_blocked: bool,
    pub shorting_enabled: bool,
    pub trade_suspended_by_user: bool,
    pub created_at: NaiveDateTime,
    pub multiplier: BigDecimal,
    pub equity: BigDecimal,
    pub last_equity: BigDecimal,
    pub long_market_value: BigDecimal,
    pub short_market_value: BigDecimal,
    pub position_market_value: BigDecimal,
    pub initial_margin: BigDecimal,
    pub maintenance_margin: BigDecimal,
    pub last_maintenance_margin: BigDecimal,
    pub sma: BigDecimal,
    pub daytrade_count: i64,
    pub crypto_tier: isize,
}
