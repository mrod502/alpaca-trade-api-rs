use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Announcement {
    pub id: String,
    pub corporate_actions_id: String,
    pub ca_type: String,
    pub ca_sub_type: String,
    pub initiating_symbol: String,
    pub initiating_original_cusip: String,
    pub target_symbol: String,
    pub target_original_cusip: String,
    pub declaration_date: String,
    pub expiration_date: String,
    pub record_date: String,
    pub payable_date: String,
    pub cash: String,
    pub old_rate: String,
    pub new_rate: String,
}
