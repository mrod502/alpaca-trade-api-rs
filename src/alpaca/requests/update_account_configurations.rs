use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct UpdateAccountConfigurations {
    pub dtbp_check: String,
    pub trade_confirm_email: String,
    pub suspend_trade: bool,
    pub no_shorting: bool,
    pub fractional_trading: bool,
}
