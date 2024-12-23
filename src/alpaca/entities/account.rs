use bigdecimal::BigDecimal;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminConfig {
    pub allow_instant_ach: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub fractional_trading: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub account_blocked: bool,
    pub account_number: String,
    pub accrued_fees: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_configurations: Option<AdminConfig>,
    pub bod_dtbp: BigDecimal,
    pub buying_power: BigDecimal,
    pub cash: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub crypto_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_tier: Option<isize>,
    pub currency: String,
    pub daytrade_count: i64,
    pub daytrading_buying_power: BigDecimal,
    pub effective_buying_power: BigDecimal,
    pub equity: BigDecimal,
    pub id: String,
    pub initial_margin: BigDecimal,
    pub last_equity: BigDecimal,
    pub last_maintenance_margin: BigDecimal,
    pub long_market_value: BigDecimal,
    pub maintenance_margin: BigDecimal,
    pub multiplier: BigDecimal,
    pub non_marginable_buying_power: BigDecimal,
    pub pattern_day_trader: bool,
    pub pending_reg_taf_fees: BigDecimal,
    pub portfolio_value: BigDecimal,
    pub position_market_value: BigDecimal,
    pub regt_buying_power: BigDecimal,
    pub short_market_value: BigDecimal,
    pub shorting_enabled: bool,
    pub sma: BigDecimal,
    pub status: String,
    pub trade_suspended_by_user: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_configurations: Option<UserConfig>,
}
