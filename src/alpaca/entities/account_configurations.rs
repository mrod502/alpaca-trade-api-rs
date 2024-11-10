use serde::{Deserialize, Serialize};

use crate::alpaca::enums::{DTBPCheck, StringEnum, TradeConfirmEmail};

#[derive(Serialize, Deserialize)]
pub struct AccountConfigurations {
    pub dtbp_check: StringEnum<DTBPCheck>,
    pub no_shorting: bool,
    pub trade_confirm_email: StringEnum<TradeConfirmEmail>,
    pub trade_suspended_by_user: bool,
}
