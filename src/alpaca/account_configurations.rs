use serde::{Deserialize, Serialize};

use super::dtbp_check::DTBPCheck;
use super::string_enum::StringEnum;
use super::trade_confirm_email::TradeConfirmEmail;

#[derive(Serialize, Deserialize)]
pub struct AccountConfigurations {
    pub dtbp_check: StringEnum<DTBPCheck>,
    pub no_shorting: bool,
    pub trade_confirm_email: StringEnum<TradeConfirmEmail>,
    pub trade_suspended_by_user: bool,
}
